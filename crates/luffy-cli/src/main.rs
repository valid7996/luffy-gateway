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
use luffy_router::{DnsFrontServer, DnsRouter};
use ports::PortsPageManager;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

fn parse_option(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

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
        "router" => {
            let rest = &args[2.min(args.len())..];
            let bind = parse_option(rest, "--bind")
                .and_then(|b| b.parse::<SocketAddr>().ok())
                .unwrap_or(SocketAddr::from(([0, 0, 0, 0], 53)));

            let mut router = DnsRouter::new();
            let mut route_count = 0usize;
            let mut idx = 0usize;
            while idx < rest.len() {
                let route_value = if rest[idx] == "--route" {
                    idx += 1;
                    rest.get(idx).cloned()
                } else if let Some(value) = rest[idx].strip_prefix("--route=") {
                    Some(value.to_string())
                } else {
                    None
                };
                idx += 1;

                let Some(value) = route_value else { continue };
                let Some((suffix, addr)) = value.split_once('=') else {
                    anyhow::bail!("--route must look like suffix=addr, got: {value}");
                };
                let backend: SocketAddr = addr
                    .parse()
                    .map_err(|e| anyhow::anyhow!("invalid backend address '{addr}': {e}"))?;
                router.add_route(suffix, backend)?;
                println!("  route {suffix:<28} -> {backend}");
                route_count += 1;
            }
            if let Some(default) = parse_option(rest, "--default").map(|d| d.parse::<SocketAddr>()) {
                match default {
                    Ok(addr) => {
                        router.set_default_backend(addr);
                        println!("  default (unmatched) -> {addr}");
                    }
                    Err(e) => anyhow::bail!("invalid --default address: {e}"),
                }
            }
            if route_count == 0 {
                println!("No --route configured. Example:");
                println!("  luffy router --bind 0.0.0.0:53 --route example.com=127.0.0.1:5310 --default 8.8.8.8:53");
            }

            println!("Starting DNS front router on {bind} ...");
            let _handle = DnsFrontServer::new(bind, router).start().await?;
            println!("Router is running. Ctrl+C to stop.");
            tokio::signal::ctrl_c().await?;
        }
        "install" => {
            println!("Starting Luffy Gateway installation preflight...");
            InstallerPreflight::check_system()?;
            let (admin_user, admin_pass, private_path) =
                InstallerPreflight::generate_admin_credentials();

            match InstallerPreflight::perform_install(8080, &private_path) {
                Ok(()) => {
                    println!("\n[SUCCESS] Installation completed successfully!");
                    println!("--------------------------------------------------");
                    println!("Panel URL:        http://<YOUR_SERVER_IP>:8080/{private_path}");
                    println!("Admin Username:   {admin_user}");
                    println!("One-Time Password: {admin_pass}");
                    println!("--------------------------------------------------");
                    println!("Notice: Change your password upon first login.");
                }
                Err(err) => {
                    println!("\n[SKIPPED] Systemd deployment not performed on this platform.");
                    println!("Reason: {err}");
                    println!("Next step: build and run locally — see INSTALL.md in the repository.");
                }
            }
        }
        "doctor" => {
            let zone = args.get(2).map(|s| s.as_str()).unwrap_or("example.com");
            let server_ip = args.get(3).cloned().unwrap_or_else(|| "127.0.0.1".into());
            println!("Running DNS Doctor diagnostics on zone: {} (server IP: {server_ip})\n", zone);
            let checks = DnsDoctor::run_diagnostics(zone, &server_ip, false);
            for c in checks {
                println!("[{:?}] {}: {}", c.status, c.name, c.detail_en);
                if let Some(fix) = c.fix_en {
                    println!("       Fix: {}", fix);
                }
            }
            println!("\nNote: checks marked WARN require the Luffy router to be running on this host.");
        }
        "ports" => {
            println!("Inspecting Luffy Gateway managed ports:\n");
            for p in PortsPageManager::get_standard_ports() {
                let in_use = PortsPageManager::is_port_in_use(p.port, &p.proto);
                let status = if in_use { "IN USE" } else { "FREE" };
                println!("{}/{} - {} [{}]", p.port, p.proto, p.service_name, status);
            }
            println!("\n(Free ports are still configured for Luffy use; firewall rules may differ.)");
        }
        "backup" => {
            println!("Creating backup manifest...");
            let manifest = BackupManager::create_manifest(
                10,
                2,
                vec!["soar".into(), "masterdns".into(), "slipgate".into(), "gooserelay".into()],
            );
            let path = BackupManager::write_manifest_file(&manifest)?;
            println!("Backup manifest written successfully: {}", path.display());
            println!("(Plain JSON preview; encryption arrives with the persistence layer.)");
        }
        _ => {
            println!("Available subcommands:");
            println!("  luffy serve [port] [--host <ip>] [--password <pw>]");
            println!("                       - Start the Web Control Panel (default: 127.0.0.1:8080)");
            println!("  luffy router [--bind addr] [--route suffix=addr]... [--default addr]");
            println!("                       - Start the live DNS front router (default bind 0.0.0.0:53)");
            println!("  luffy install        - Run production installer (Linux systemd; graceful elsewhere)");
            println!("  luffy doctor <zone> [server-ip] - Run 7-point DNS Doctor diagnostics");
            println!("  luffy ports          - Probe firewall & managed ports");
            println!("  luffy backup         - Write a backup manifest file");
            println!("  luffy help           - Display this help message");
        }
    }

    Ok(())
}
