use crate::traits::Engine;
use async_trait::async_trait;
use luffy_core::models::{EngineKind, Entitlement, User};
use luffy_core::{CoreError, RiskLevel, SettingEntry, SettingScope, SettingType};
use std::collections::HashMap;

pub struct GooseRelayAdapter {
    pinned_version: &'static str,
}

impl Default for GooseRelayAdapter {
    fn default() -> Self {
        Self {
            pinned_version: "v1.1.0",
        }
    }
}

impl GooseRelayAdapter {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl Engine for GooseRelayAdapter {
    fn kind(&self) -> EngineKind {
        EngineKind::GooseRelay
    }

    fn pinned_version(&self) -> &'static str {
        self.pinned_version
    }

    fn settings_schema(&self) -> Vec<SettingEntry> {
        vec![
            SettingEntry {
                key: "exit_port".to_string(),
                engine: EngineKind::GooseRelay,
                scope: SettingScope::Server,
                val_type: SettingType::Integer,
                default_value: serde_json::json!(8443),
                range: Some((1024, 65535)),
                unit: Some("port".to_string()),
                help_en: "VPS exit server TCP port reachable by Google Apps Script".to_string(),
                help_fa: "پورت تی‌سی‌پی سرور خروجی جهت برقراری ارتباط از سمت گوگل".to_string(),
                risk: RiskLevel::Careful,
                restart_required: true,
                upstream_key_or_flag: "port".to_string(),
            },
            SettingEntry {
                key: "tunnel_key".to_string(),
                engine: EngineKind::GooseRelay,
                scope: SettingScope::Server,
                val_type: SettingType::String,
                default_value: serde_json::json!(""),
                range: None,
                unit: None,
                help_en: "Pre-shared authorization key between Apps Script and VPS".to_string(),
                help_fa: "کلید محرمانه احراز هویت بین گوگل اپ اسکریپت و سرور خروجی".to_string(),
                risk: RiskLevel::Dangerous,
                restart_required: true,
                upstream_key_or_flag: "tunnel_key".to_string(),
            },
            SettingEntry {
                key: "workers".to_string(),
                engine: EngineKind::GooseRelay,
                scope: SettingScope::Client,
                val_type: SettingType::Integer,
                default_value: serde_json::json!(8),
                range: Some((1, 32)),
                unit: Some("threads".to_string()),
                help_en: "Concurrent worker threads in the GooseRelay client".to_string(),
                help_fa: "تعداد ورکرها و رشته‌های موازی در نرم‌افزار کلاینت".to_string(),
                risk: RiskLevel::Safe,
                restart_required: false,
                upstream_key_or_flag: "workers".to_string(),
            },
        ]
    }

    fn render_server_config(
        &self,
        settings: &HashMap<String, serde_json::Value>,
    ) -> Result<String, CoreError> {
        let port = settings
            .get("exit_port")
            .and_then(|v| v.as_i64())
            .unwrap_or(8443);
        let key = settings
            .get("tunnel_key")
            .and_then(|v| v.as_str())
            .unwrap_or("auto_generated_key");

        Ok(format!(
            "{{\n  \"listen_port\": {port},\n  \"tunnel_key\": \"{key}\",\n  \"metrics_enabled\": true\n}}"
        ))
    }

    fn render_client_config(
        &self,
        _settings: &HashMap<String, serde_json::Value>,
        _user: &User,
        entitlement: &Entitlement,
    ) -> Result<String, CoreError> {
        let script_id = entitlement
            .client_options
            .get("script_id")
            .and_then(|v| v.as_str())
            .unwrap_or("AKfycb...");

        Ok(format!(
            "{{\n  \"socks5_listen\": \"127.0.0.1:1080\",\n  \"script_keys\": [\"{script_id}\"],\n  \"google_host\": \"script.google.com\",\n  \"workers\": 8\n}}"
        ))
    }

    fn validate_config(&self, raw_config: &str) -> Result<(), CoreError> {
        if !raw_config.contains("listen_port") {
            return Err(CoreError::Validation("Missing 'listen_port'".into()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gooserelay_config_rendering() {
        let adapter = GooseRelayAdapter::new();
        let mut settings = HashMap::new();
        settings.insert("exit_port".into(), serde_json::json!(9443));
        settings.insert("tunnel_key".into(), serde_json::json!("key123"));

        let config = adapter.render_server_config(&settings).unwrap();
        assert!(config.contains("\"listen_port\": 9443"));
        assert!(config.contains("\"tunnel_key\": \"key123\""));
        assert!(adapter.validate_config(&config).is_ok());
    }
}
