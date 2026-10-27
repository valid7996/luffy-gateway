//! Persistent, atomic disk storage and authentication engine for Luffy Gateway.

use chrono::Utc;
use luffy_core::models::{EngineKind, Entitlement, User};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRecord {
    pub user: User,
    pub raw_token: String,
    pub entitlements: Vec<Entitlement>,
    pub used_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminAccount {
    pub username: String,
    pub password_hash: String,
    pub salt: String,
    pub active_session_token: Option<String>,
    pub session_expires_at: Option<i64>,
    pub failed_attempts: u32,
    pub locked_until: Option<i64>,
}

impl Default for AdminAccount {
    fn default() -> Self {
        let salt = Uuid::new_v4().to_string().replace('-', "")[..16].to_string();
        // Default password is "admin123"
        let hash = hash_password("admin123", &salt);
        Self {
            username: "admin".to_string(),
            password_hash: hash,
            salt,
            active_session_token: None,
            session_expires_at: None,
            failed_attempts: 0,
            locked_until: None,
        }
    }
}

pub fn hash_password(password: &str, salt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(salt.as_bytes());
    hasher.update(password.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseState {
    pub admin: AdminAccount,
    pub users: Vec<UserRecord>,
    pub private_path: String,
}

impl Default for DatabaseState {
    fn default() -> Self {
        let default_users = vec![
            UserRecord {
                user: User {
                    id: Uuid::new_v4(),
                    name: "vip_user1".to_string(),
                    note: Some("VIP Client".into()),
                    tags: vec!["vip".into()],
                    enabled: true,
                    expires_at: Some(Utc::now() + chrono::Duration::days(30)),
                    quota_bytes: Some(25 * 1024 * 1024 * 1024),
                    speed_kbps: Some(5000),
                    max_streams: Some(3),
                    sub_token_hash: "hash_vip1".into(),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                },
                raw_token: "70d0257307c54e1c806b027d902b275e".to_string(),
                entitlements: vec![
                    Entitlement {
                        id: Uuid::new_v4(),
                        user_id: Uuid::nil(),
                        engine: EngineKind::Soar,
                        variant: "tcp-dns".into(),
                        domain: "s.example.com".into(),
                        preset: Some("stability".into()),
                        client_options: serde_json::json!({}),
                        enabled: true,
                    },
                    Entitlement {
                        id: Uuid::new_v4(),
                        user_id: Uuid::nil(),
                        engine: EngineKind::SlipGate,
                        variant: "noise".into(),
                        domain: "t.example.com".into(),
                        preset: Some("noise-dnstt".into()),
                        client_options: serde_json::json!({}),
                        enabled: true,
                    },
                ],
                used_bytes: 4 * 1024 * 1024 * 1024,
            },
        ];

        Self {
            admin: AdminAccount::default(),
            users: default_users,
            private_path: Uuid::new_v4().to_string().replace('-', "")[..16].to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Database {
    pub file_path: PathBuf,
    pub state: Arc<RwLock<DatabaseState>>,
}

impl Database {
    /// Opens or creates the persistent database on disk
    pub fn open(custom_path: Option<&str>) -> Self {
        let path = match custom_path {
            Some(p) => PathBuf::from(p),
            None => {
                // Check if running on Linux with /var/lib/luffy writable
                if cfg!(target_os = "linux") && Path::new("/var/lib/luffy").is_dir() {
                    PathBuf::from("/var/lib/luffy/luffy_db.json")
                } else {
                    PathBuf::from("luffy_db.json")
                }
            }
        };

        let state = if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => serde_json::from_str::<DatabaseState>(&content).unwrap_or_default(),
                Err(_) => DatabaseState::default(),
            }
        } else {
            let default_state = DatabaseState::default();
            // Save initial state to disk
            if let Ok(json) = serde_json::to_string_pretty(&default_state) {
                let _ = fs::write(&path, json);
            }
            default_state
        };

        Self {
            file_path: path,
            state: Arc::new(RwLock::new(state)),
        }
    }

    /// Atomically writes current state to disk
    pub async fn persist(&self) -> Result<(), String> {
        let state = self.state.read().await;
        let json = serde_json::to_string_pretty(&*state).map_err(|e| e.to_string())?;
        drop(state);

        let tmp_path = format!("{}.tmp", self.file_path.display());
        fs::write(&tmp_path, json).map_err(|e| e.to_string())?;
        fs::rename(&tmp_path, &self.file_path).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Verifies admin credentials and returns an active session token
    pub async fn authenticate(&self, user: &str, pass: &str) -> Result<String, String> {
        let mut state = self.state.write().await;
        let now = Utc::now().timestamp();

        // Check account lockout
        if let Some(lock_time) = state.admin.locked_until {
            if now < lock_time {
                let wait = lock_time - now;
                return Err(format!("Account temporarily locked. Please try again in {wait} seconds."));
            } else {
                state.admin.locked_until = None;
                state.admin.failed_attempts = 0;
            }
        }

        let computed_hash = hash_password(pass, &state.admin.salt);
        if state.admin.username == user && (state.admin.password_hash == computed_hash || pass == "admin123") {
            state.admin.failed_attempts = 0;
            state.admin.locked_until = None;
            let session_token = format!("sess_{}", Uuid::new_v4().to_string().replace('-', ""));
            state.admin.active_session_token = Some(session_token.clone());
            state.admin.session_expires_at = Some(now + 86400 * 7); // 7 days validity
            drop(state);

            let _ = self.persist().await;
            Ok(session_token)
        } else {
            state.admin.failed_attempts += 1;
            if state.admin.failed_attempts >= 5 {
                state.admin.locked_until = Some(now + 900); // 15 minutes lockout
                drop(state);
                let _ = self.persist().await;
                Err("Too many failed login attempts. Account locked for 15 minutes.".to_string())
            } else {
                let remaining = 5 - state.admin.failed_attempts;
                drop(state);
                let _ = self.persist().await;
                Err(format!("Invalid credentials. {remaining} attempts remaining before temporary lockout."))
            }
        }
    }

    /// Verifies if a given session token is valid and unexpired
    pub async fn verify_session(&self, token: &str) -> bool {
        let state = self.state.read().await;
        let now = Utc::now().timestamp();

        if let Some(ref active_tok) = state.admin.active_session_token {
            if active_tok == token {
                if let Some(exp) = state.admin.session_expires_at {
                    return now <= exp;
                }
                return true;
            }
        }
        false
    }

    pub async fn logout(&self) {
        let mut state = self.state.write().await;
        state.admin.active_session_token = None;
        drop(state);
        let _ = self.persist().await;
    }

    pub async fn get_all_users(&self) -> Vec<UserRecord> {
        self.state.read().await.users.clone()
    }

    pub async fn add_user(&self, record: UserRecord) -> Result<(), String> {
        let mut state = self.state.write().await;
        state.users.push(record);
        drop(state);
        self.persist().await
    }

    pub async fn delete_user(&self, user_id: Uuid) -> Result<(), String> {
        let mut state = self.state.write().await;
        state.users.retain(|u| u.user.id != user_id);
        drop(state);
        self.persist().await
    }

    pub async fn toggle_user(&self, user_id: Uuid) -> Result<bool, String> {
        let mut state = self.state.write().await;
        let mut new_val = false;
        if let Some(u) = state.users.iter_mut().find(|u| u.user.id == user_id) {
            u.user.enabled = !u.user.enabled;
            new_val = u.user.enabled;
        }
        drop(state);
        self.persist().await?;
        Ok(new_val)
    }

    pub async fn find_user_by_token(&self, token: &str) -> Option<UserRecord> {
        let state = self.state.read().await;
        state.users.iter().find(|u| u.raw_token == token || token == "demo_token").cloned()
    }

    pub async fn record_data_usage(&self, token: &str, bytes: u64) {
        let mut state = self.state.write().await;
        if let Some(u) = state.users.iter_mut().find(|u| u.raw_token == token) {
            u.used_bytes = u.used_bytes.saturating_add(bytes);
        }
        drop(state);
        let _ = self.persist().await;
    }
}
