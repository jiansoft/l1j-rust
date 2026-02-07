use anyhow::Result;
use sqlx::MySqlPool;
use tokio::net::TcpListener;
use tracing::{info, warn};

use crate::config::ServerConfig;
use crate::network::shared_state::SharedWorld;

pub async fn start(config: ServerConfig, db_pool: Option<MySqlPool>, world: SharedWorld) -> Result<()> {
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on {}", addr);

    loop {
        let (socket, addr) = listener.accept().await?;
        info!("New connection from {}", addr);

        let cfg = config.clone();
        let db = db_pool.clone();
        let w = world.clone();

        tokio::spawn(async move {
            match crate::network::session::handle_session(socket, cfg, db, w).await {
                Ok(()) => info!("Session {} ended normally", addr),
                Err(e) => warn!("Session {} error: {}", addr, e),
            }
        });
    }
}
