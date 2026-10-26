//! Luffy Core: Domain models, settings schema, error types, and validation logic.

pub mod error;
pub mod models;
pub mod registry;

pub use error::CoreError;
pub use registry::{RiskLevel, SettingEntry, SettingScope, SettingType, SettingsRegistry};
