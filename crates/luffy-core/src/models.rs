use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EngineKind {
    Soar,
    MasterDns,
    SlipGate,
    GooseRelay,
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
