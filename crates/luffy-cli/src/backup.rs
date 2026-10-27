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

    /// Writes the manifest to disk next to the working directory and returns
    /// the concrete file path. (Encryption arrives with the persistence layer.)
    pub fn write_manifest_file(manifest: &BackupManifest) -> Result<std::path::PathBuf, CoreError> {
        let stamp = Utc::now().format("%Y%m%d-%H%M%S");
        let path = std::env::current_dir()
            .map_err(|e| CoreError::Internal(e.to_string()))?
            .join(format!("luffy-backup-{stamp}.json"));

        let json = serde_json::to_string_pretty(manifest)
            .map_err(|e| CoreError::Internal(e.to_string()))?;
        std::fs::write(&path, json).map_err(|e| CoreError::Internal(format!("Backup write failed: {e}")))?;
        Ok(path)
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

    #[test]
    fn test_manifest_file_is_written() {
        let manifest = BackupManager::create_manifest(3, 1, vec!["soar".into()]);
        let path = BackupManager::write_manifest_file(&manifest).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("\"users_count\": 3"));
        // Cleanup the artifact of this test run
        let _ = std::fs::remove_file(&path);
    }
}
