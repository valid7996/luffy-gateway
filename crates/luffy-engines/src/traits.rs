use async_trait::async_trait;
use luffy_core::models::{EngineKind, Entitlement, User};
use luffy_core::{CoreError, SettingEntry};
use std::collections::HashMap;

#[async_trait]
pub trait Engine: Send + Sync {
    fn kind(&self) -> EngineKind;
    fn pinned_version(&self) -> &'static str;
    fn settings_schema(&self) -> Vec<SettingEntry>;

    fn render_server_config(
        &self,
        settings: &HashMap<String, serde_json::Value>,
    ) -> Result<String, CoreError>;

    fn render_client_config(
        &self,
        settings: &HashMap<String, serde_json::Value>,
        user: &User,
        entitlement: &Entitlement,
    ) -> Result<String, CoreError>;

    fn validate_config(&self, raw_config: &str) -> Result<(), CoreError>;
}

#[async_trait]
pub trait ServiceManager: Send + Sync {
    async fn start(&self, name: &str) -> Result<(), CoreError>;
    async fn stop(&self, name: &str) -> Result<(), CoreError>;
    async fn restart(&self, name: &str) -> Result<(), CoreError>;
    async fn is_active(&self, name: &str) -> Result<bool, CoreError>;
}
