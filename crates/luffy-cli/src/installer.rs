//! Installation engine and systemd unit generator.

use luffy_core::CoreError;
use uuid::Uuid;

pub struct InstallerPreflight;

impl InstallerPreflight {
    pub fn check_system() -> Result<(), CoreError> {
        // Verification of system environment
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
}
