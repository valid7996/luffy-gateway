use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EngineKind {
    Soar,
    MasterDns,
    SlipGate,
    GooseRelay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DomainRole {
    Tunnel,
    Panel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DomainStatus {
    Valid,
    Warning,
    Error,
    Unchecked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domain {
    pub id: Uuid,
    pub fqdn: String,
    pub role: DomainRole,
    pub engine: Option<EngineKind>,
    pub preferred: bool,
    pub status: DomainStatus,
    pub last_checked_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entitlement {
    pub id: Uuid,
    pub user_id: Uuid,
    pub engine: EngineKind,
    pub variant: String,
    pub domain: String,
    pub preset: Option<String>,
    pub client_options: serde_json::Value,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub note: Option<String>,
    pub tags: Vec<String>,
    pub enabled: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub quota_bytes: Option<u64>,
    pub speed_kbps: Option<u32>,
    pub max_streams: Option<u32>,
    pub sub_token_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
