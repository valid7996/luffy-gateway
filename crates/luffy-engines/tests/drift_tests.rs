use luffy_core::models::EngineKind;
use luffy_core::SettingsRegistry;
use luffy_engines::{Engine, MasterDnsAdapter, SoarAdapter};

#[test]
fn test_settings_registry_drift_soar() {
    let adapter = SoarAdapter::new();
    let schema = adapter.settings_schema();
    assert!(!schema.is_empty(), "Soar schema must not be empty");

    let mut registry = SettingsRegistry::new();
    for entry in schema {
        assert_eq!(entry.engine, EngineKind::Soar);
        assert!(!entry.help_en.is_empty(), "English help must not be empty");
        assert!(!entry.help_fa.is_empty(), "Persian help must not be empty");
        registry.register(entry);
    }

    assert!(registry.get(EngineKind::Soar, "qname_max_len").is_some());
    assert!(registry.get(EngineKind::Soar, "listen_addr").is_some());
}

#[test]
fn test_settings_registry_drift_masterdns() {
    let adapter = MasterDnsAdapter::new();
    let schema = adapter.settings_schema();
    assert!(!schema.is_empty(), "MasterDns schema must not be empty");

    let mut registry = SettingsRegistry::new();
    for entry in schema {
        assert_eq!(entry.engine, EngineKind::MasterDns);
        assert!(!entry.help_en.is_empty(), "English help must not be empty");
        assert!(!entry.help_fa.is_empty(), "Persian help must not be empty");
        registry.register(entry);
    }

    assert!(registry.get(EngineKind::MasterDns, "encryption").is_some());
    assert!(registry.get(EngineKind::MasterDns, "balancing_strategy").is_some());
}
