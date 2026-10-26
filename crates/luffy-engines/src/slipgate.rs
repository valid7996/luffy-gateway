use crate::slipnet_codec::{SlipNetCodec, SlipNetProfile};
use crate::traits::Engine;
use async_trait::async_trait;
use luffy_core::models::{EngineKind, Entitlement, User};
use luffy_core::{CoreError, RiskLevel, SettingEntry, SettingScope, SettingType};
use std::collections::HashMap;

pub struct SlipGateAdapter {
    pinned_version: &'static str,
}

impl Default for SlipGateAdapter {
    fn default() -> Self {
        Self {
            pinned_version: "v1.6.2",
        }
    }
}

impl SlipGateAdapter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Builds the non-interactive CLI arguments for adding a tunnel
    pub fn build_cli_tunnel_add(
        &self,
        transport: &str,
        backend: &str,
        domain: &str,
        listen_port: u16,
    ) -> Vec<String> {
        vec![
            "tunnel".into(),
            "add".into(),
            "--transport".into(),
            transport.into(),
            "--backend".into(),
            backend.into(),
            "--domain".into(),
            domain.into(),
            "--port".into(),
            listen_port.to_string(),
        ]
    }
}

#[async_trait]
impl Engine for SlipGateAdapter {
    fn kind(&self) -> EngineKind {
        EngineKind::SlipGate
    }

    fn pinned_version(&self) -> &'static str {
        self.pinned_version
    }

    fn settings_schema(&self) -> Vec<SettingEntry> {
        vec![
            SettingEntry {
                key: "transport".to_string(),
                engine: EngineKind::SlipGate,
                scope: SettingScope::Server,
                val_type: SettingType::Enum(vec![
                    "dnstt".to_string(),
                    "vaydns".to_string(),
                    "slipstream".to_string(),
                    "naive".to_string(),
                    "stuntls".to_string(),
                    "direct-ssh".to_string(),
                    "direct-socks5".to_string(),
                ]),
                default_value: serde_json::json!("dnstt"),
                range: None,
                unit: None,
                help_en: "SlipGate tunnel transport layer".to_string(),
                help_fa: "لایه انتقال و پروتکل تونل SlipGate".to_string(),
                risk: RiskLevel::Safe,
                restart_required: true,
                upstream_key_or_flag: "--transport".to_string(),
            },
            SettingEntry {
                key: "backend".to_string(),
                engine: EngineKind::SlipGate,
                scope: SettingScope::Server,
                val_type: SettingType::Enum(vec![
                    "socks".to_string(),
                    "ssh".to_string(),
                    "both".to_string(),
                ]),
                default_value: serde_json::json!("socks"),
                range: None,
                unit: None,
                help_en: "Internal proxy backend for tunnel termination".to_string(),
                help_fa: "بک‌اند خاتمه‌دهنده ترافیک داخلی (ساکس یا اس‌اس‌اچ)".to_string(),
                risk: RiskLevel::Safe,
                restart_required: true,
                upstream_key_or_flag: "--backend".to_string(),
            },
            SettingEntry {
                key: "query_size_preset".to_string(),
                engine: EngineKind::SlipGate,
                scope: SettingScope::Client,
                val_type: SettingType::Enum(vec![
                    "100".to_string(),
                    "80".to_string(),
                    "60".to_string(),
                    "50".to_string(),
                ]),
                default_value: serde_json::json!("80"),
                range: None,
                unit: Some("bytes".to_string()),
                help_en: "DNS query size preset for NoizDNS/DNSTT DPI evasion".to_string(),
                help_fa: "سایز بسته کوئری دی‌ان‌اس جهت دور زدن سیستم‌های فیلترینگ عمیق (DPI)".to_string(),
                risk: RiskLevel::Safe,
                restart_required: false,
                upstream_key_or_flag: "--query-size".to_string(),
            },
            SettingEntry {
                key: "utls_fingerprint".to_string(),
                engine: EngineKind::SlipGate,
                scope: SettingScope::Client,
                val_type: SettingType::Enum(vec![
                    "chrome_120".to_string(),
                    "firefox_120".to_string(),
                    "safari_16".to_string(),
                    "randomized".to_string(),
                ]),
                default_value: serde_json::json!("chrome_120"),
                range: None,
                unit: None,
                help_en: "uTLS TLS ClientHello fingerprint camouflage".to_string(),
                help_fa: "اثرانگشت شبیه‌سازی تی‌ال‌اس کلاینت برای عدم شناسایی توسط فایروال".to_string(),
                risk: RiskLevel::Safe,
                restart_required: false,
                upstream_key_or_flag: "--utls".to_string(),
            },
        ]
    }

    fn render_server_config(
        &self,
        settings: &HashMap<String, serde_json::Value>,
    ) -> Result<String, CoreError> {
        let transport = settings
            .get("transport")
            .and_then(|v| v.as_str())
            .unwrap_or("dnstt");
        let backend = settings
            .get("backend")
            .and_then(|v| v.as_str())
            .unwrap_or("socks");

        Ok(format!(
            "{{\n  \"transport\": \"{transport}\",\n  \"backend\": \"{backend}\",\n  \"multi_mode\": true\n}}"
        ))
    }

    fn render_client_config(
        &self,
        _settings: &HashMap<String, serde_json::Value>,
        _user: &User,
        entitlement: &Entitlement,
    ) -> Result<String, CoreError> {
        let qsize = entitlement
            .client_options
            .get("query_size_preset")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(80);

        let utls = entitlement
            .client_options
            .get("utls_fingerprint")
            .and_then(|v| v.as_str())
            .unwrap_or("chrome_120")
            .to_string();

        let profile = SlipNetProfile {
            tag: format!("Luffy {}", entitlement.variant),
            transport: entitlement.variant.clone(),
            domain: entitlement.domain.clone(),
            public_key: None,
            resolver: Some("1.1.1.1:53".to_string()),
            transport_protocol: "udp".to_string(),
            query_size: Some(qsize),
            utls_fingerprint: Some(utls),
            ssh_user: None,
            ssh_password: None,
            socks_user: None,
            socks_password: None,
        };

        Ok(SlipNetCodec::encode(&profile))
    }

    fn validate_config(&self, raw_config: &str) -> Result<(), CoreError> {
        if !raw_config.contains("transport") {
            return Err(CoreError::Validation("Missing 'transport' field in SlipGate config".into()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slipgate_cli_args_generation() {
        let adapter = SlipGateAdapter::new();
        let args = adapter.build_cli_tunnel_add("dnstt", "socks", "t.example.com", 5330);
        assert_eq!(args[0], "tunnel");
        assert_eq!(args[1], "add");
        assert_eq!(args[3], "dnstt");
        assert_eq!(args[5], "socks");
        assert_eq!(args[7], "t.example.com");
        assert_eq!(args[9], "5330");
    }
}
