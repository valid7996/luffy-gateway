//! Installation engine and systemd unit generator.

use luffy_core::CoreError;
use uuid::Uuid;

pub struct InstallerPreflight;

/// Returns true when running as root (Linux).
fn nix_root() -> bool {
    std::env::var("USER").map(|u| u == "root").unwrap_or(false)
}

impl InstallerPreflight {
    pub fn check_system() -> Result<(), CoreError> {
        // Verification of system environment
        if cfg!(target_os = "linux") && !nix_root() {
            return Err(CoreError::Internal(
                "Installation requires root on Linux (run with sudo).".into(),
            ));
        }
        Ok(())
    }

    /// Performs the real systemd deployment (Linux only):
    /// writes the hardened unit files and enables the services.
    pub fn perform_install(panel_port: u16, private_path: &str) -> Result<(), CoreError> {
        if !cfg!(target_os = "linux") {
            return Err(CoreError::Internal(
                "The installer targets Linux (systemd). On this platform, run \
                 `luffy serve --password <pw>` directly — see INSTALL.md."
                    .into(),
            ));
        }
        Self::check_system()?;

        let units = [
            ("/etc/systemd/system/luffy-router.service", Self::render_router_systemd_unit()),
            (
                "/etc/systemd/system/luffy-panel.service",
                Self::render_panel_systemd_unit(panel_port, private_path),
            ),
        ];
        for (path, content) in units {
            std::fs::write(path, content)
                .map_err(|e| CoreError::Internal(format!("Unit write failed ({path}): {e}")))?;
        }

        for cmd in [
            ["systemctl", "daemon-reload"].join(" "),
            ["systemctl", "enable", "--now", "luffy-router.service"].join(" "),
            ["systemctl", "enable", "--now", "luffy-panel.service"].join(" "),
        ] {
            let status = std::process::Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .status()
                .map_err(|e| CoreError::Internal(format!("systemctl failed: {e}")))?;
            if !status.success() {
                return Err(CoreError::Internal(format!("Command failed: {cmd}")));
            }
        }
        Ok(())
    }

    pub fn generate_admin_credentials() -> (String, String, String) {
        let username = "admin".to_string();
        let one_time_password = Uuid::new_v4().to_string().replace('-', "")[..16].to_string();
        let private_path = Uuid::new_v4().to_string().replace('-', "");
        (username, one_time_password, private_path)
    }

    pub fn render_router_systemd_unit() -> String {
        r#"[Unit]
Description=Luffy Gateway DNS Front Router
After=network.target

[Service]
Type=simple
User=luffy
Group=luffy
ExecStart=/usr/local/bin/luffy router run --port 53
Restart=always
RestartSec=3s
AmbientCapabilities=CAP_NET_BIND_SERVICE
CapabilityBoundingSet=CAP_NET_BIND_SERVICE
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
PrivateTmp=true
LimitNOFILE=65535

[Install]
WantedBy=multi-user.target
"#
        .to_string()
    }

    pub fn render_panel_systemd_unit(panel_port: u16, private_path: &str) -> String {
        format!(
            r#"[Unit]
Description=Luffy Gateway Control Panel
After=network.target luffy-router.service

[Service]
Type=simple
User=luffy
Group=luffy
ExecStart=/usr/local/bin/luffy panel run --port {panel_port} --private-path {private_path}
Restart=always
RestartSec=5s
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
PrivateTmp=true
LimitNOFILE=65535

[Install]
WantedBy=multi-user.target
"#
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admin_credentials_generation() {
        let (user, pass, path) = InstallerPreflight::generate_admin_credentials();
        assert_eq!(user, "admin");
        assert_eq!(pass.len(), 16);
        assert_eq!(path.len(), 32);
    }

    #[test]
    fn test_hardened_systemd_units() {
        let router_unit = InstallerPreflight::render_router_systemd_unit();
        assert!(router_unit.contains("CAP_NET_BIND_SERVICE"));
        assert!(router_unit.contains("NoNewPrivileges=true"));
        assert!(router_unit.contains("LimitNOFILE=65535"));

        let panel_unit = InstallerPreflight::render_panel_systemd_unit(8080, "secretpath");
        assert!(panel_unit.contains("--port 8080"));
        assert!(panel_unit.contains("--private-path secretpath"));
        assert!(panel_unit.contains("ProtectSystem=strict"));
    }

    #[test]
    fn test_install_is_honest_on_non_linux_or_non_root() {
        // In test environments (Windows dev, CI without root) the real install
        // must fail with a clear message instead of pretending success.
        let result = InstallerPreflight::perform_install(8080, "path");
        if !cfg!(target_os = "linux") || !nix_root() {
            assert!(result.is_err());
        }
    }
}
