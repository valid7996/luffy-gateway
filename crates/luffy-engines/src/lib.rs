//! Luffy Engines: Adapter trait and implementations for Soar, MasterDnsVPN, SlipGate, GooseRelay.

use async_trait::async_trait;
use luffy_core::models::EngineKind;
use luffy_core::CoreError;

#[async_trait]
pub trait Engine: Send + Sync {
    fn kind(&self) -> EngineKind;
    fn pinned_version(&self) -> &'static str;
    async fn install(&self) -> Result<(), CoreError>;
    async fn start(&self) -> Result<(), CoreError>;
    async fn stop(&self) -> Result<(), CoreError>;
    async fn status(&self) -> Result<String, CoreError>;
}
