//! Luffy CLI entrypoint

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    println!("Luffy Gateway v{}", env!("CARGO_PKG_VERSION"));
    println!("Type 'luffy help' for commands or run installer.");
    Ok(())
}
