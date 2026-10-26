use crate::traits::Engine;
use async_trait::async_trait;
use luffy_core::models::{EngineKind, Entitlement, User};
use luffy_core::{CoreError, RiskLevel, SettingEntry, SettingScope, SettingType};
use std::collections::HashMap;

pub struct SoarAdapter {
    pinned_version: &'static str,
}

impl Default for SoarAdapter {
    fn default() -> Self {
        Self {
            pinned_version: "v0.8.2",
        }
    }
}

impl SoarAdapter {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl Engine for SoarAdapter {
    fn kind(&self) -> EngineKind {
        EngineKind::Soar
    }

    fn pinned_version(&self) -> &'static str {
        self.pinned_version
    }

    fn settings_schema(&self) -> Vec<SettingEntry> {
        vec![
            SettingEntry {
                key: "listen_addr".to_string(),
                engine: EngineKind::Soar,
                scope: SettingScope::Server,
                val_type: SettingType::String,
                default_value: serde_json::json!("127.0.0.1:5310"),
                range: None,
                unit: None,
                help_en: "Loopback listen address behind the Luffy DNS front router".to_string(),
                help_fa: "آدرس گوش دادن محلی در پشت روتر فرانت دی‌ان‌اس Luffy".to_string(),
                risk: RiskLevel::Careful,
                restart_required: true,
                upstream_key_or_flag: "listen".to_string(),
            },
            SettingEntry {
                key: "qname_min_len".to_string(),
                engine: EngineKind::Soar,
                scope: SettingScope::Server,
                val_type: SettingType::Integer,
                default_value: serde_json::json!(101),
                range: Some((50, 255)),
                unit: Some("bytes".to_string()),
                help_en: "Initial QNAME length in DNS request".to_string(),
                help_fa: "حداقل طول ابتدایی نام دامنه QNAME در درخواست".to_string(),
                risk: RiskLevel::Safe,
                restart_required: false,
                upstream_key_or_flag: "qname-min-len".to_string(),
            },
            SettingEntry {
                key: "qname_max_len".to_string(),
                engine: EngineKind::Soar,
                scope: SettingScope::Server,
                val_type: SettingType::Integer,
                default_value: serde_json::json!(253),
                range: Some((100, 255)),
                unit: Some("bytes".to_string()),
                help_en: "Maximum QNAME length in DNS request".to_string(),
                help_fa: "حداکثر طول نام دامنه QNAME در درخواست‌های دی‌ان‌اس".to_string(),
                risk: RiskLevel::Safe,
                restart_required: false,
                upstream_key_or_flag: "qname-max-len".to_string(),
            },
            SettingEntry {
                key: "max_answer_size".to_string(),
                engine: EngineKind::Soar,
                scope: SettingScope::Server,
                val_type: SettingType::Integer,
                default_value: serde_json::json!(1232),
                range: Some((512, 4096)),
                unit: Some("bytes".to_string()),
                help_en: "Maximum DNS answer size (EDNS UDP buffer)".to_string(),
                help_fa: "حداکثر سایز پاسخ دی‌ان‌اس (بافر EDNS)".to_string(),
                risk: RiskLevel::Careful,
                restart_required: false,
                upstream_key_or_flag: "max-answer-size".to_string(),
            },
            SettingEntry {
                key: "stream_limit".to_string(),
                engine: EngineKind::Soar,
                scope: SettingScope::Server,
                val_type: SettingType::Integer,
                default_value: serde_json::json!(128),
                range: Some((1, 1024)),
                unit: Some("streams".to_string()),
                help_en: "Maximum concurrent TCP streams per account".to_string(),
                help_fa: "حداکثر تعداد استریم همزمان TCP برای هر اکانت".to_string(),
                risk: RiskLevel::Safe,
                restart_required: false,
                upstream_key_or_flag: "stream-limit".to_string(),
            },
            SettingEntry {
                key: "idle_timeout_secs".to_string(),
                engine: EngineKind::Soar,
                scope: SettingScope::Server,
                val_type: SettingType::Integer,
                default_value: serde_json::json!(60),
                range: Some((10, 600)),
                unit: Some("seconds".to_string()),
                help_en: "TCP stream idle timeout in seconds".to_string(),
                help_fa: "مدت زمان غیرفعال بودن مجاز استریم TCP قبل از بسته شدن".to_string(),
                risk: RiskLevel::Safe,
                restart_required: false,
                upstream_key_or_flag: "idle-timeout".to_string(),
            },
            SettingEntry {
                key: "preset".to_string(),
                engine: EngineKind::Soar,
                scope: SettingScope::Profile,
                val_type: SettingType::Enum(vec![
                    "stability".to_string(),
                    "balanced".to_string(),
                    "performance".to_string(),
                    "limited_path".to_string(),
                ]),
                default_value: serde_json::json!("stability"),
                range: None,
                unit: None,
                help_en: "Pre-configured tuning profile for Iranian networks".to_string(),
                help_fa: "پروفایل بهینه‌سازی تنظیمات متناسب با وضعیت شبکه ایران".to_string(),
                risk: RiskLevel::Safe,
                restart_required: false,
                upstream_key_or_flag: "preset".to_string(),
            },
        ]
    }

    fn render_server_config(
        &self,
        settings: &HashMap<String, serde_json::Value>,
    ) -> Result<String, CoreError> {
        let listen_addr = settings
            .get("listen_addr")
            .and_then(|v| v.as_str())
            .unwrap_or("127.0.0.1:5310");
        let qname_min = settings
            .get("qname_min_len")
            .and_then(|v| v.as_i64())
            .unwrap_or(101);
        let qname_max = settings
            .get("qname_max_len")
            .and_then(|v| v.as_i64())
            .unwrap_or(253);
        let max_answer = settings
            .get("max_answer_size")
            .and_then(|v| v.as_i64())
            .unwrap_or(1232);
        let stream_limit = settings
            .get("stream_limit")
            .and_then(|v| v.as_i64())
            .unwrap_or(128);
        let idle_timeout = settings
            .get("idle_timeout_secs")
            .and_then(|v| v.as_i64())
            .unwrap_or(60);

        Ok(format!(
            "# Luffy Generated Soar Server Configuration\n\
            listen = \"{listen_addr}\"\n\
            qname_min_len = {qname_min}\n\
            qname_max_len = {qname_max}\n\
            max_answer_size = {max_answer}\n\
            stream_limit = {stream_limit}\n\
            idle_timeout = {idle_timeout}\n\
            tcp_only = true\n"
        ))
    }

    fn render_client_config(
        &self,
        _settings: &HashMap<String, serde_json::Value>,
        _user: &User,
        entitlement: &Entitlement,
    ) -> Result<String, CoreError> {
        let domain = &entitlement.domain;
        let local_port = 10850;

        Ok(format!(
            "# Soar Client Configuration for Luffy\n\
            [client]\n\
            domain = \"{domain}\"\n\
            local_socks5 = \"127.0.0.1:{local_port}\"\n\
            max_answer_size = 1232\n\
            queries_per_path = 32\n\
            in_flight_queries = 128\n"
        ))
    }

    fn validate_config(&self, raw_config: &str) -> Result<(), CoreError> {
        if !raw_config.contains("listen") {
            return Err(CoreError::Validation("Missing 'listen' directive".into()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soar_config_rendering() {
        let adapter = SoarAdapter::new();
        let mut settings = HashMap::new();
        settings.insert("listen_addr".into(), serde_json::json!("127.0.0.1:5311"));
        settings.insert("qname_min_len".into(), serde_json::json!(120));

        let config = adapter.render_server_config(&settings).unwrap();
        assert!(config.contains("listen = \"127.0.0.1:5311\""));
        assert!(config.contains("qname_min_len = 120"));
        assert!(adapter.validate_config(&config).is_ok());
    }
}
