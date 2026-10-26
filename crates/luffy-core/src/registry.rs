use crate::models::EngineKind;
use crate::CoreError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SettingScope {
    Server,
    Client,
    Profile,
    Panel,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SettingType {
    String,
    Integer,
    Boolean,
    Enum(Vec<String>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    Safe,
    Careful,
    Dangerous,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingEntry {
    pub key: String,
    pub engine: EngineKind,
    pub scope: SettingScope,
    pub val_type: SettingType,
    pub default_value: serde_json::Value,
    pub range: Option<(i64, i64)>,
    pub unit: Option<String>,
    pub help_en: String,
    pub help_fa: String,
    pub risk: RiskLevel,
    pub restart_required: bool,
    pub upstream_key_or_flag: String,
}

impl SettingEntry {
    pub fn validate_value(&self, value: &serde_json::Value) -> Result<(), CoreError> {
        match &self.val_type {
            SettingType::Boolean => {
                if !value.is_boolean() {
                    return Err(CoreError::Validation(format!(
                        "Key '{}' expects a boolean, found {}",
                        self.key, value
                    )));
                }
            }
            SettingType::String => {
                if !value.is_string() {
                    return Err(CoreError::Validation(format!(
                        "Key '{}' expects a string, found {}",
                        self.key, value
                    )));
                }
            }
            SettingType::Integer => {
                if let Some(num) = value.as_i64() {
                    if let Some((min, max)) = self.range {
                        if num < min || num > max {
                            return Err(CoreError::Validation(format!(
                                "Key '{}' out of range [{min}, {max}]: {num}",
                                self.key
                            )));
                        }
                    }
                } else {
                    return Err(CoreError::Validation(format!(
                        "Key '{}' expects an integer, found {}",
                        self.key, value
                    )));
                }
            }
            SettingType::Enum(variants) => {
                if let Some(s) = value.as_str() {
                    if !variants.iter().any(|v| v.eq_ignore_ascii_case(s)) {
                        return Err(CoreError::Validation(format!(
                            "Key '{}' invalid enum value '{}'. Allowed: {:?}",
                            self.key, s, variants
                        )));
                    }
                } else {
                    return Err(CoreError::Validation(format!(
                        "Key '{}' expects an enum string, found {}",
                        self.key, value
                    )));
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SettingsRegistry {
    entries: Vec<SettingEntry>,
}

impl SettingsRegistry {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn register(&mut self, entry: SettingEntry) {
        self.entries.retain(|e| !(e.engine == entry.engine && e.key == entry.key));
        self.entries.push(entry);
    }

    pub fn get(&self, engine: EngineKind, key: &str) -> Option<&SettingEntry> {
        self.entries.iter().find(|e| e.engine == engine && e.key == key)
    }

    pub fn for_engine(&self, engine: EngineKind) -> Vec<&SettingEntry> {
        self.entries.iter().filter(|e| e.engine == engine).collect()
    }

    pub fn all_entries(&self) -> &[SettingEntry] {
        &self.entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setting_entry_validation() {
        let entry = SettingEntry {
            key: "qname_max_len".to_string(),
            engine: EngineKind::Soar,
            scope: SettingScope::Server,
            val_type: SettingType::Integer,
            default_value: serde_json::json!(253),
            range: Some((100, 255)),
            unit: Some("bytes".to_string()),
            help_en: "Maximum QNAME length in DNS request".to_string(),
            help_fa: "حداکثر طول QNAME در درخواست‌های دی‌ان‌اس".to_string(),
            risk: RiskLevel::Careful,
            restart_required: true,
            upstream_key_or_flag: "qname-max-len".to_string(),
        };

        assert!(entry.validate_value(&serde_json::json!(200)).is_ok());
        assert!(entry.validate_value(&serde_json::json!(300)).is_err());
        assert!(entry.validate_value(&serde_json::json!("invalid")).is_err());
    }
}
