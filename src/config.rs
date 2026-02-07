use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub server: ServerSection,
    pub database: DatabaseSection,
    pub game: GameSection,
    #[serde(default = "default_paths")]
    pub paths: PathsSection,
}

fn default_paths() -> PathsSection {
    PathsSection {
        maps_dir: "../L1J-TW_3.80c/maps".to_string(),
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSection {
    pub host: String,
    pub port: u16,
    pub max_online_users: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSection {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GameSection {
    pub tick_interval_ms: u64,
    pub npc_ai_sleep_range: u32,
    pub packet_batch_flush: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PathsSection {
    pub maps_dir: String,
}

impl ServerConfig {
    pub fn load(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path))?;
        let config: ServerConfig =
            toml::from_str(&content).with_context(|| "Failed to parse config file")?;
        Ok(config)
    }
}
