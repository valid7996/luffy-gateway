//! Subscription handler and profile exporter for /sub/<token>.

use chrono::Utc;
use luffy_core::models::{EngineKind, Entitlement, User};
use luffy_core::CoreError;
use luffy_engines::{Engine, GooseRelayAdapter, MasterDnsAdapter, SlipGateAdapter, SoarAdapter};
use luffy_ui::escape_html;
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
                let configs: Vec<serde_json::Value> = profiles
                    .into_iter()
                    .filter(|p| p.engine == EngineKind::GooseRelay)
                    .filter_map(|p| serde_json::from_str(&p.config_payload).ok())
                    .collect();
                let json = serde_json::to_string(&configs)
                    .map_err(|e| CoreError::Internal(e.to_string()))?;
                Ok(("application/json".to_string(), json))
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
        let title = if is_fa { "پورتال اشتراک شخصی Luffy" } else { "Luffy Subscription Portal" };
        let usage_label = if is_fa { "مصرف حجم" } else { "Data Usage" };
        let user_label = if is_fa { "نام کاربری" } else { "Subscriber" };
        let copy_label = if is_fa { "کپی کانفیگ" } else { "Copy Config" };
        let copied_label = if is_fa { "کپی شد!" } else { "Copied!" };
        let _profiles_count = profiles.len();

        let total_quota_str = user
            .quota_bytes
            .map(|q| format!(" / {} MB", q / (1024 * 1024)))
            .unwrap_or_else(|| "".to_string());

        let used_mb = used_bytes / (1024 * 1024);
        let percent = user
            .quota_bytes
            .map(|q| {
                if q > 0 {
                    ((used_bytes as f64 / q as f64) * 100.0).min(100.0) as u32
                } else {
                    0
                }
            })
            .unwrap_or(25);

        let user_name = escape_html(&user.name);
        let user_label = escape_html(user_label);
        let usage_label = escape_html(usage_label);
        let copy_label = escape_html(copy_label);
        let copied_label = escape_html(copied_label);

        let mut cards = String::new();
        for (idx, p) in profiles.iter().enumerate() {
            let engine_title = format!("{:?} · {}", p.engine, escape_html(&p.variant));
            let domain = escape_html(&p.domain);
            let payload = escape_html(&p.config_payload);
            let textarea_id = format!("cfg_{}", idx);
            cards.push_str(&format!(
                r#"<div class="config-card">
                    <div class="config-header">
                        <div>
                            <div class="config-title">{engine_title}</div>
                            <div class="config-domain">{domain}</div>
                        </div>
                        <button class="btn-copy" onclick="copyConfig('{textarea_id}', this)">
                            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect width="14" height="14" x="8" y="8" rx="2" ry="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/></svg>
                            <span>{copy_label}</span>
                        </button>
                    </div>
                    <textarea id="{textarea_id}" readonly rows="3">{payload}</textarea>
                </div>"#,
                engine_title = engine_title,
                domain = domain,
                payload = payload,
                textarea_id = textarea_id,
                copy_label = copy_label
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
        :root {{
            --bg-base: #090d16;
            --bg-card: #0f172a;
            --bg-input: #040810;
            --border: #1e293b;
            --border-highlight: #334155;
            --text-main: #f8fafc;
            --text-muted: #94a3b8;
            --text-dim: #64748b;
            --primary: #38bdf8;
            --primary-glow: rgba(56, 189, 248, 0.15);
            --success: #10b981;
            --radius: 12px;
        }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Vazirmatn', sans-serif;
            background: var(--bg-base);
            color: var(--text-main);
            padding: 40px 16px;
            min-height: 100vh;
            display: flex;
            justify-content: center;
        }}
        .container {{ width: 100%; max-width: 640px; }}
        .header-card {{
            background: var(--bg-card);
            border: 1px solid var(--border);
            border-radius: var(--radius);
            padding: 24px;
            margin-bottom: 20px;
            box-shadow: 0 8px 32px rgba(0,0,0,0.4);
        }}
        .brand-pill {{
            display: inline-flex;
            align-items: center;
            gap: 6px;
            font-size: 0.75rem;
            color: var(--primary);
            font-weight: 700;
            text-transform: uppercase;
            letter-spacing: 0.05em;
            margin-bottom: 12px;
        }}
        h1 {{ font-size: 1.4rem; font-weight: 700; margin-bottom: 16px; }}
        .info-grid {{ display: grid; grid-template-columns: 1fr 1fr; gap: 12px; margin-bottom: 16px; }}
        .info-item {{ background: rgba(255,255,255,0.02); border: 1px solid var(--border); padding: 12px; border-radius: 8px; }}
        .info-label {{ font-size: 0.75rem; color: var(--text-dim); text-transform: uppercase; margin-bottom: 4px; }}
        .info-val {{ font-size: 1.1rem; font-weight: 600; color: var(--text-main); }}
        .meter-container {{ margin-top: 12px; }}
        .meter-track {{ width: 100%; height: 6px; background: var(--bg-input); border-radius: 3px; overflow: hidden; }}
        .meter-fill {{ width: {percent}%; height: 100%; background: linear-gradient(90deg, #0ea5e9, #38bdf8); border-radius: 3px; }}

        .config-card {{
            background: var(--bg-card);
            border: 1px solid var(--border);
            border-radius: var(--radius);
            padding: 18px;
            margin-bottom: 14px;
            transition: all 0.2s ease;
        }}
        .config-card:hover {{ border-color: var(--border-highlight); }}
        .config-header {{ display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px; }}
        .config-title {{ font-size: 0.95rem; font-weight: 600; color: var(--text-main); }}
        .config-domain {{ font-size: 0.775rem; color: var(--text-dim); font-family: monospace; }}
        .btn-copy {{
            display: inline-flex;
            align-items: center;
            gap: 6px;
            background: var(--primary-glow);
            border: 1px solid rgba(56, 189, 248, 0.3);
            color: var(--primary);
            padding: 6px 12px;
            border-radius: 6px;
            font-size: 0.775rem;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.2s;
        }}
        .btn-copy:hover {{ background: rgba(56, 189, 248, 0.25); }}
        textarea {{
            width: 100%;
            background: var(--bg-input);
            color: #7dd3fc;
            border: 1px solid var(--border);
            border-radius: 6px;
            padding: 10px;
            font-family: monospace;
            font-size: 0.8rem;
            resize: none;
            outline: none;
        }}
    </style>
    <script>
        function copyConfig(id, btn) {{
            let ta = document.getElementById(id);
            if (!ta) return;
            navigator.clipboard.writeText(ta.value).then(() => {{
                let orig = btn.innerHTML;
                btn.innerHTML = '<span>{copied_label}</span>';
                setTimeout(() => {{ btn.innerHTML = orig; }}, 1800);
            }});
        }}
    </script>
</head>
<body>
    <div class="container">
        <div class="header-card">
            <div class="brand-pill">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M12 2v20"/><path d="M5 12h14"/><circle cx="12" cy="12" r="9"/></svg>
                <span>Luffy Subscription Portal</span>
            </div>
            <h1>{title}</h1>
            <div class="info-grid">
                <div class="info-item">
                    <div class="info-label">{user_label}</div>
                    <div class="info-val">{user_name}</div>
                </div>
                <div class="info-item">
                    <div class="info-label">{usage_label}</div>
                    <div class="info-val">{used_mb} MB{total_quota_str}</div>
                </div>
            </div>
            <div class="meter-container">
                <div style="display:flex; justify-content:space-between; font-size:0.75rem; color:var(--text-dim); margin-bottom:4px;">
                    <span>Bandwidth Meter</span>
                    <span>{percent}%</span>
                </div>
                <div class="meter-track">
                    <div class="meter-fill"></div>
                </div>
            </div>
        </div>
        {cards}
    </div>
</body>
</html>"#,
            lang = lang,
            dir = dir,
            title = title,
            user_label = user_label,
            user_name = user_name,
            usage_label = usage_label,
            used_mb = used_mb,
            total_quota_str = total_quota_str,
            percent = percent,
            cards = cards,
            copied_label = copied_label
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
