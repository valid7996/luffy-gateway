//! Localization (i18n) manager for Persian (FA) and English (EN).

use std::collections::HashMap;

const FA_CATALOG_RAW: &str = include_str!("../locales/fa.json");
const EN_CATALOG_RAW: &str = include_str!("../locales/en.json");

pub struct I18nManager {
    fa_strings: HashMap<String, String>,
    en_strings: HashMap<String, String>,
}

impl Default for I18nManager {
    fn default() -> Self {
        let fa_strings: HashMap<String, String> =
            serde_json::from_str(FA_CATALOG_RAW).expect("Valid Persian locale JSON");
        let en_strings: HashMap<String, String> =
            serde_json::from_str(EN_CATALOG_RAW).expect("Valid English locale JSON");

        Self {
            fa_strings,
            en_strings,
        }
    }
}

impl I18nManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Translates key into the target language, falling back to English, then to the key itself.
    pub fn translate(&self, lang: &str, key: &str) -> String {
        let catalog = if lang.starts_with("fa") {
            &self.fa_strings
        } else {
            &self.en_strings
        };

        if let Some(val) = catalog.get(key) {
            val.clone()
        } else if let Some(fallback) = self.en_strings.get(key) {
            fallback.clone()
        } else {
            key.to_string()
        }
    }

    pub fn fa_keys(&self) -> impl Iterator<Item = &String> {
        self.fa_strings.keys()
    }

    pub fn en_keys(&self) -> impl Iterator<Item = &String> {
        self.en_strings.keys()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i18n_catalog_symmetry() {
        let manager = I18nManager::new();

        for k in manager.en_keys() {
            assert!(
                manager.fa_strings.contains_key(k),
                "Missing key '{k}' in Persian catalog"
            );
        }

        for k in manager.fa_keys() {
            assert!(
                manager.en_strings.contains_key(k),
                "Missing key '{k}' in English catalog"
            );
        }
    }

    #[test]
    fn test_translation_lookups() {
        let manager = I18nManager::new();
        assert_eq!(manager.translate("fa", "dashboard"), "داشبورد");
        assert_eq!(manager.translate("en", "dashboard"), "Dashboard");
    }
}
