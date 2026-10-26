use crate::traits::Engine;
use async_trait::async_trait;
use luffy_core::models::{EngineKind, Entitlement, User};
use luffy_core::{CoreError, RiskLevel, SettingEntry, SettingScope, SettingType};
use std::collections::HashMap;

pub struct MasterDnsAdapter {
    pinned_version: &'static str,
}

impl Default for MasterDnsAdapter {
    fn default() -> Self {
        Self {
            pinned_version: "v1.2.0",
        }
    }
}

impl MasterDnsAdapter {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl Engine for MasterDnsAdapter {
    fn kind(&self) -> EngineKind {
        EngineKind::MasterDns
    }

    fn pinned_version(&self) -> &'static str {
        self.pinned_version
    }

    fn settings_schema(&self) -> Vec<SettingEntry> {
        vec![
            SettingEntry {
                key: "listen_addr".to_string(),
                engine: EngineKind::MasterDns,
                scope: SettingScope::Server,
                val_type: SettingType::String,
                default_value: serde_json::json!("127.0.0.1:5320"),
                range: None,
                unit: None,
                help_en: "Loopback listen address behind the Luffy DNS front router".to_string(),
                help_fa: "آدرس گوش دادن محلی در پشت روتر فرانت دی‌ان‌اس Luffy".to_string(),
                risk: RiskLevel::Careful,
                restart_required: true,
                upstream_key_or_flag: "LISTEN_ADDR".to_string(),
            },
            SettingEntry {
                key: "encryption".to_string(),
                engine: EngineKind::MasterDns,
                scope: SettingScope::Server,
                val_type: SettingType::Enum(vec![
                    "None".to_string(),
                    "XOR".to_string(),
                    "ChaCha20".to_string(),
                    "AES-GCM".to_string(),
                ]),
                default_value: serde_json::json!("ChaCha20"),
                range: None,
                unit: None,
                help_en: "Data payload encryption method".to_string(),
                help_fa: "الگوریتم رمزنگاری داده‌های ارسالی".to_string(),
                risk: RiskLevel::Safe,
                restart_required: true,
                upstream_key_or_flag: "ENCRYPTION".to_string(),
            },
            SettingEntry {
                key: "mtu".to_string(),
                engine: EngineKind::MasterDns,
                scope: SettingScope::Server,
                val_type: SettingType::Integer,
                default_value: serde_json::json!(1200),
                range: Some((500, 1500)),
                unit: Some("bytes".to_string()),
                help_en: "Tunnel MTU to prevent packet fragmentation".to_string(),
                help_fa: "حداکثر واحد انتقال پکت (MTU) برای جلوگیری از تکه‌تکه شدن بسته‌ها".to_string(),
                risk: RiskLevel::Careful,
                restart_required: false,
                upstream_key_or_flag: "MTU".to_string(),
            },
            SettingEntry {
                key: "workers".to_string(),
                engine: EngineKind::MasterDns,
                scope: SettingScope::Server,
                val_type: SettingType::Integer,
                default_value: serde_json::json!(4),
                range: Some((1, 32)),
                unit: Some("threads".to_string()),
                help_en: "Number of worker threads processing packets".to_string(),
                help_fa: "تعداد پردازنده‌های موازی رسیدگی به بسته‌ها".to_string(),
                risk: RiskLevel::Safe,
                restart_required: true,
                upstream_key_or_flag: "WORKERS".to_string(),
            },
            SettingEntry {
                key: "balancing_strategy".to_string(),
                engine: EngineKind::MasterDns,
                scope: SettingScope::Client,
                val_type: SettingType::Enum(vec![
                    "RoundRobin".to_string(),
                    "Random".to_string(),
                    "Fastest".to_string(),
                    "LeastLatency".to_string(),
                    "Weighted".to_string(),
                    "Priority".to_string(),
                    "Fallback".to_string(),
                    "DuplicateAll".to_string(),
                ]),
                default_value: serde_json::json!("LeastLatency"),
                range: None,
                unit: None,
                help_en: "Client resolver balancing and routing strategy across public resolvers".to_string(),
                help_fa: "استراتژی بالانس و انتخاب ریزالورهای دی‌ان‌اس توسط کلاینت".to_string(),
                risk: RiskLevel::Safe,
                restart_required: false,
                upstream_key_or_flag: "BALANCING_STRATEGY".to_string(),
            },
            SettingEntry {
                key: "use_external_socks5".to_string(),
                engine: EngineKind::MasterDns,
                scope: SettingScope::Server,
                val_type: SettingType::Boolean,
                default_value: serde_json::json!(false),
                range: None,
                unit: None,
                help_en: "Route decrypted egress traffic through external SOCKS5 proxy or WARP".to_string(),
                help_fa: "هدایت ترافیک خروجی از طریق پراکسی خارجی SOCKS5 یا وارپ".to_string(),
                risk: RiskLevel::Careful,
                restart_required: true,
                upstream_key_or_flag: "USE_EXTERNAL_SOCKS5".to_string(),
            },
        ]
    }

    fn render_server_config(
        &self,
        settings: &HashMap<String, serde_json::Value>,
    ) -> Result<String, CoreError> {
        let listen = settings
            .get("listen_addr")
            .and_then(|v| v.as_str())
            .unwrap_or("127.0.0.1:5320");
        let encryption = settings
            .get("encryption")
            .and_then(|v| v.as_str())
            .unwrap_or("ChaCha20");
        let mtu = settings
            .get("mtu")
            .and_then(|v| v.as_i64())
            .unwrap_or(1200);
        let workers = settings
            .get("workers")
            .and_then(|v| v.as_i64())
            .unwrap_or(4);
        let ext_socks = settings
            .get("use_external_socks5")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        Ok(format!(
            "# Luffy Generated MasterDnsVPN Server Configuration\n\
            LISTEN_ADDR = \"{listen}\"\n\
            ENCRYPTION = \"{encryption}\"\n\
            MTU = {mtu}\n\
            WORKERS = {workers}\n\
            USE_EXTERNAL_SOCKS5 = {ext_socks}\n"
        ))
    }

    fn render_client_config(
        &self,
        _settings: &HashMap<String, serde_json::Value>,
        _user: &User,
        entitlement: &Entitlement,
    ) -> Result<String, CoreError> {
        let domain = &entitlement.domain;
        let strategy = entitlement
            .client_options
            .get("balancing_strategy")
            .and_then(|v| v.as_str())
            .unwrap_or("LeastLatency");

        Ok(format!(
            "# MasterDnsVPN Client Configuration\n\
            SERVER_DOMAIN = \"{domain}\"\n\
            BALANCING_STRATEGY = \"{strategy}\"\n\
            ENCRYPTION = \"ChaCha20\"\n\
            LOCAL_SOCKS5 = \"127.0.0.1:10860\"\n"
        ))
    }

    fn validate_config(&self, raw_config: &str) -> Result<(), CoreError> {
        if !raw_config.contains("LISTEN_ADDR") {
            return Err(CoreError::Validation("Missing 'LISTEN_ADDR'".into()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_masterdns_config_rendering() {
        let adapter = MasterDnsAdapter::new();
        let mut settings = HashMap::new();
        settings.insert("encryption".into(), serde_json::json!("AES-GCM"));
        settings.insert("mtu".into(), serde_json::json!(1150));

        let config = adapter.render_server_config(&settings).unwrap();
        assert!(config.contains("ENCRYPTION = \"AES-GCM\""));
        assert!(config.contains("MTU = 1150"));
        assert!(adapter.validate_config(&config).is_ok());
    }
}
