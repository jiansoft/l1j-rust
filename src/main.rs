use anyhow::Result;
use l1j_rust::config;
use l1j_rust::db;
use l1j_rust::network;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();

    println!("┌───────────────────────────────┐");
    println!("│   L1J-Rust Server v0.1.0      │");
    println!("│   Lineage 1 (3.80c TW)        │");
    println!("└───────────────────────────────┘");

    let config = config::ServerConfig::load("config/server.toml")?;
    info!("Config: {}:{}, max_users={}", config.server.host, config.server.port, config.server.max_online_users);

    let db_pool = match db::pool::create_pool(&config.database).await {
        Ok(pool) => {
            info!("Database connected");
            Some(pool)
        }
        Err(e) => {
            warn!("Database failed: {}. Running without DB.", e);
            None
        }
    };

    // Create shared world state (lets players see each other)
    let world = network::shared_state::create_shared_world();
    info!("Shared world initialized");

    info!("=== Server ready ===");
    network::listener::start(config, db_pool, world).await?;

    Ok(())
}
