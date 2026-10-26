//! Subscription handler and profile exporter for /sub/<token>.

use chrono::Utc;
use luffy_core::models::{EngineKind, Entitlement, User};
use luffy_core::CoreError;
use luffy_engines::{Engine, GooseRelayAdapter, MasterDnsAdapter, SlipGateAdapter, SoarAdapter};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubFormat {
    Luffy,
    SlipNet,
    Soar,
    MasterDns,
    Goose,
    Html,
}

impl SubFormat {
    pub fn from_query(q: Option<&str>) -> Self {
        match q.unwrap_or("html").to_lowercase().as_str() {
            "luffy" => Self::Luffy,
            "slipnet" => Self::SlipNet,
            "soar" => Self::Soar,
            "masterdns" => Self::MasterDns,
            "goose" => Self::Goose,
            _ => Self::Html,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LuffySubscriptionBundle {
    pub user_name: String,
    pub quota_bytes: Option<u64>,
    pub used_bytes: u64,
    pub expires_at: Option<i64>,
    pub profiles: Vec<LuffyEntitlementProfile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LuffyEntitlementProfile {
    pub engine: EngineKind,
    pub variant: String,
    pub domain: String,
    pub config_payload: String,
}

pub struct SubscriptionService {
    soar: SoarAdapter,
    masterdns: MasterDnsAdapter,
    slipgate: SlipGateAdapter,
    goose: GooseRelayAdapter,
}

impl Default for SubscriptionService {
    fn default() -> Self {
        Self {
            soar: SoarAdapter::new(),
            masterdns: MasterDnsAdapter::new(),
            slipgate: SlipGateAdapter::new(),
            goose: GooseRelayAdapter::new(),
        }
    }
}

impl SubscriptionService {
    pub fn new() -> Self {
        Self::default()
    }

    /// Generates Subscription-Userinfo header value
    pub fn build_userinfo_header(used_bytes: u64, quota_bytes: Option<u64>, expires_at: Option<i64>) -> String {
        let upload = used_bytes / 2;
        let download = used_bytes - upload;
        let total = quota_bytes.unwrap_or(0);
        let expire = expires_at.unwrap_or(0);

        format!("upload={upload}; download={download}; total={total}; expire={expire}")
    }

    /// Renders subscription output according to the requested format
    pub fn render_subscription(
        &self,
        user: &User,
        entitlements: &[Entitlement],
        used_bytes: u64,
        format: SubFormat,
        lang: &str,
    ) -> Result<(String, String), CoreError> {
        // Enforce account active status
        if !user.enabled {
            return Err(CoreError::Validation("Account is disabled".into()));
        }

        if let Some(exp) = user.expires_at {
            if Utc::now() >= exp {
                return Err(CoreError::Validation("Account has expired".into()));
            }
        }

        if let Some(quota) = user.quota_bytes {
            if used_bytes >= quota {
                return Err(CoreError::Validation("Account traffic quota exhausted".into()));
            }
        }

        let empty_settings = HashMap::new();
        let mut profiles = Vec::new();

        for ent in entitlements {
            if !ent.enabled {
                continue;
            }

            let payload = match ent.engine {
                EngineKind::Soar => self.soar.render_client_config(&empty_settings, user, ent)?,
                EngineKind::MasterDns => {
                    self.masterdns
                        .render_client_config(&empty_settings, user, ent)?
                }
                EngineKind::SlipGate => {
                    self.slipgate
                        .render_client_config(&empty_settings, user, ent)?
                }
                EngineKind::GooseRelay => {
                    self.goose
                        .render_client_config(&empty_settings, user, ent)?
                }
            };

            profiles.push(LuffyEntitlementProfile {
                engine: ent.engine,
                variant: ent.variant.clone(),
                domain: ent.domain.clone(),
                config_payload: payload,
            });
        }

        match format {
            SubFormat::Luffy => {
                let bundle = LuffySubscriptionBundle {
                    user_name: user.name.clone(),
                    quota_bytes: user.quota_bytes,
                    used_bytes,
                    expires_at: user.expires_at.map(|dt| dt.timestamp()),
                    profiles,
                };
                let json = serde_json::to_string_pretty(&bundle)
                    .map_err(|e| CoreError::Internal(e.to_string()))?;
                Ok(("application/json".to_string(), json))
            }
            SubFormat::SlipNet => {
                let uris: Vec<String> = profiles
                    .into_iter()
                    .filter(|p| p.engine == EngineKind::SlipGate)
                    .map(|p| p.config_payload)
                    .collect();
                Ok(("text/plain; charset=utf-8".to_string(), uris.join("\n")))
            }
            SubFormat::Soar => {
                let configs: Vec<String> = profiles
                    .into_iter()
                    .filter(|p| p.engine == EngineKind::Soar)
                    .map(|p| p.config_payload)
                    .collect();
                Ok(("text/plain; charset=utf-8".to_string(), configs.join("\n---\n")))
            }
            SubFormat::MasterDns => {
                let configs: Vec<String> = profiles
                    .into_iter()
                    .filter(|p| p.engine == EngineKind::MasterDns)
                    .map(|p| p.config_payload)
                    .collect();
                Ok(("text/plain; charset=utf-8".to_string(), configs.join("\n---\n")))
            }
            SubFormat::Goose => {
                let configs: Vec<String> = profiles
                    .into_iter()
                    .filter(|p| p.engine == EngineKind::GooseRelay)
                    .map(|p| p.config_payload)
                    .collect();
                Ok(("application/json".to_string(), configs.join("\n")))
            }
            SubFormat::Html => {
                let html = self.render_landing_page(user, &profiles, used_bytes, lang);
                Ok(("text/html; charset=utf-8".to_string(), html))
            }
        }
    }

    fn render_landing_page(
        &self,
        user: &User,
        profiles: &[LuffyEntitlementProfile],
        used_bytes: u64,
        lang: &str,
    ) -> String {
        let is_fa = lang.starts_with("fa");
        let dir = if is_fa { "rtl" } else { "ltr" };
        let title = if is_fa { "پورتال اشتراک Luffy" } else { "Luffy Subscription Portal" };
        let usage_label = if is_fa { "حجم مصرفی" } else { "Traffic Usage" };
        let user_label = if is_fa { "کاربر" } else { "User" };
        let profiles_count = profiles.len();

        let total_quota_str = user
            .quota_bytes
            .map(|q| format!(" / {} MB", q / (1024 * 1024)))
            .unwrap_or_else(|| "".to_string());

        let used_mb = used_bytes / (1024 * 1024);

        let mut cards = String::new();
        for p in profiles {
            let engine_title = format!("{:?} - {}", p.engine, p.variant);
            cards.push_str(&format!(
                r#"<div class="card">
                    <h3>{engine_title}</h3>
                    <p><code>{domain}</code></p>
                    <textarea readonly rows="3">{payload}</textarea>
                </div>"#,
                domain = p.domain,
                payload = p.config_payload
            ));
        }

        format!(
            r#"<!DOCTYPE html>
<html lang="{lang}" dir="{dir}">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>{title}</title>
    <style>
        body {{ font-family: system-ui, -apple-system, sans-serif; background: #0f172a; color: #f8fafc; margin: 0; padding: 20px; }}
        .container {{ max-width: 600px; margin: 0 auto; }}
        .status-box {{ background: #1e293b; padding: 20px; border-radius: 12px; margin-bottom: 24px; border: 1px solid #334155; }}
        .card {{ background: #1e293b; padding: 16px; border-radius: 8px; margin-bottom: 16px; border: 1px solid #334155; }}
        textarea {{ width: 100%; box-sizing: border-box; background: #090d16; color: #38bdf8; border: 1px solid #475569; border-radius: 4px; padding: 8px; font-family: monospace; resize: none; }}
        code {{ color: #a78bfa; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="status-box">
            <h2>{title}</h2>
            <p>{user_label}: <strong>{user_name}</strong></p>
            <p>{usage_label}: <strong>{used_mb} MB{total_quota_str}</strong></p>
            <p>Active Profiles: <strong>{profiles_count}</strong></p>
        </div>
        {cards}
    </div>
</body>
</html>"#,
            lang = lang,
            dir = dir,
            title = title,
            user_label = user_label,
            user_name = user.name,
            usage_label = usage_label,
            used_mb = used_mb,
            total_quota_str = total_quota_str,
            profiles_count = profiles_count,
            cards = cards
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_subscription_userinfo_header() {
        let header = SubscriptionService::build_userinfo_header(1000, Some(5000), Some(1730000000));
        assert_eq!(header, "upload=500; download=500; total=5000; expire=1730000000");
    }

    #[test]
    fn test_subscription_rendering_luffy_bundle() {
        let service = SubscriptionService::new();
        let user = User {
            id: Uuid::new_v4(),
            name: "test_subscriber".to_string(),
            note: None,
            tags: vec![],
            enabled: true,
            expires_at: None,
            quota_bytes: Some(10_000_000),
            speed_kbps: None,
            max_streams: None,
            sub_token_hash: "hash".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let ent = Entitlement {
            id: Uuid::new_v4(),
            user_id: user.id,
            engine: EngineKind::SlipGate,
            variant: "dnstt".to_string(),
            domain: "t.example.com".to_string(),
            preset: None,
            client_options: serde_json::json!({ "query_size_preset": "80" }),
            enabled: true,
        };

        let (mime, content) = service
            .render_subscription(&user, &[ent], 2_000_000, SubFormat::Luffy, "en")
            .unwrap();

        assert_eq!(mime, "application/json");
        assert!(content.contains("\"user_name\": \"test_subscriber\""));
        assert!(content.contains("slipnet://"));
    }

    #[test]
    fn test_disabled_user_rejected() {
        let service = SubscriptionService::new();
        let user = User {
            id: Uuid::new_v4(),
            name: "disabled_user".to_string(),
            note: None,
            tags: vec![],
            enabled: false,
            expires_at: None,
            quota_bytes: None,
            speed_kbps: None,
            max_streams: None,
            sub_token_hash: "hash".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let res = service.render_subscription(&user, &[], 0, SubFormat::Luffy, "en");
        assert!(res.is_err());
    }
}
