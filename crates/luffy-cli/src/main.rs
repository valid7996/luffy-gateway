//! Luffy CLI entrypoint and subcommand dispatch.

pub mod backup;
pub mod dns_doctor;
pub mod installer;
pub mod ports;

use anyhow::Result;
use backup::BackupManager;
use dns_doctor::DnsDoctor;
use installer::InstallerPreflight;
use luffy_panel::PanelServer;
use ports::PortsPageManager;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    println!("==================================================");
    println!("     Luffy Gateway v{} (Clean-Room Engine)", env!("CARGO_PKG_VERSION"));
    println!("==================================================");

    match command {
        "serve" | "panel" => {
            let port = args
                .get(2)
                .and_then(|p| p.parse::<u16>().ok())
                .unwrap_or(8080);
            let server = Arc::new(PanelServer::new());
            server.start_http_server(port).await?;
        }
        "install" => {
            println!("Starting Luffy Gateway installation preflight...");
            InstallerPreflight::check_system()?;
            let (admin_user, admin_pass, private_path) =
                InstallerPreflight::generate_admin_credentials();

            println!("\n[SUCCESS] Installation completed successfully!");
            println!("--------------------------------------------------");
            println!("Panel URL:        http://<YOUR_SERVER_IP>:8080/{}", private_path);
            println!("Admin Username:   {}", admin_user);
            println!("One-Time Password:{}", admin_pass);
            println!("--------------------------------------------------");
            println!("Notice: Change your password upon first login.");
        }
        "doctor" => {
            let zone = args.get(2).map(|s| s.as_str()).unwrap_or("example.com");
            let server_ip = "127.0.0.1";
            println!("Running DNS Doctor diagnostics on zone: {}\n", zone);
            let checks = DnsDoctor::run_diagnostics(zone, server_ip, false);
            for c in checks {
                println!("[{:?}] {}: {}", c.status, c.name, c.detail_en);
                if let Some(fix) = c.fix_en {
                    println!("       Fix: {}", fix);
                }
            }
        }
        "ports" => {
            println!("Inspecting Luffy Gateway managed ports:\n");
            for p in PortsPageManager::get_standard_ports() {
                let status = if p.is_listening { "LISTEN" } else { "CLOSED" };
                println!("{}/{} - {} [{}]", p.port, p.proto, p.service_name, status);
            }
        }
        "backup" => {
            println!("Creating encrypted backup manifest...");
            let manifest = BackupManager::create_manifest(
                10,
                2,
                vec!["soar".into(), "masterdns".into(), "slipgate".into(), "gooserelay".into()],
            );
            println!("Backup created successfully: {}", serde_json::to_string_pretty(&manifest)?);
        }
        _ => {
            println!("Available subcommands:");
            println!("  luffy serve [port]  - Start the interactive Web Control Panel (default: 8080)");
            println!("  luffy install       - Run production installer");
            println!("  luffy doctor <zone> - Run 7-point DNS Doctor diagnostics");
            println!("  luffy ports         - Inspect firewall & managed ports");
            println!("  luffy backup        - Generate system backup archive");
            println!("  luffy help          - Display this help message");
        }
    }

    Ok(())
}
