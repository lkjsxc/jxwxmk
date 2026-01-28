pub mod loader;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub tick_rate: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WorldConfig {
    pub seed: u32,
    pub chunk_size: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GameConfig {
    pub server: ServerConfig,
    pub world: WorldConfig,
}
