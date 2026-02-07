use anyhow::Result;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;

use crate::config::DatabaseSection;

/// Create an async MySQL connection pool.
///
/// Uses sqlx with tokio runtime. The pool lazily creates connections
/// up to `max_connections`.
pub async fn create_pool(config: &DatabaseSection) -> Result<MySqlPool> {
    let pool = MySqlPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.url)
        .await?;

    // Verify the connection works
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await?;

    Ok(pool)
}
