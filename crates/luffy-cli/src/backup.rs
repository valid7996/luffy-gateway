//! Encrypted backup and restore previewer for Luffy Gateway.

use chrono::Utc;
use luffy_core::CoreError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupManifest {
    pub version: String,
    pub created_at: String,
    pub users_count: usize,
    pub domains_count: usize,
    pub engines_active: Vec<String>,
}

pub struct BackupManager;

impl BackupManager {
    pub fn create_manifest(users: usize, domains: usize, engines: Vec<String>) -> BackupManifest {
        BackupManifest {
            version: "0.1.0".to_string(),
            created_at: Utc::now().to_rfc3339(),
            users_count: users,
            domains_count: domains,
            engines_active: engines,
        }
    }

    pub fn preview_restore(manifest_json: &str) -> Result<String, CoreError> {
        let manifest: BackupManifest = serde_json::from_str(manifest_json)
            .map_err(|e| CoreError::Validation(format!("Corrupt backup manifest: {e}")))?;

        Ok(format!(
            "Backup Preview:\n- Luffy Version: {}\n- Created At: {}\n- Users to restore: {}\n- Domains to restore: {}\n- Engines: {:?}",
            manifest.version, manifest.created_at, manifest.users_count, manifest.domains_count, manifest.engines_active
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_and_preview() {
        let manifest = BackupManager::create_manifest(
            15,
            3,
            vec!["soar".into(), "masterdns".into(), "slipgate".into(), "gooserelay".into()],
        );
        let json = serde_json::to_string(&manifest).unwrap();
        let preview = BackupManager::preview_restore(&json).unwrap();

        assert!(preview.contains("Users to restore: 15"));
        assert!(preview.contains("Domains to restore: 3"));
    }
}
