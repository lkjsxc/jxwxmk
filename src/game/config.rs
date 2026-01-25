use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub tick_rate: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GameConfig {
    pub world_width: f64,
    pub world_height: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub game: GameConfig,
}

impl AppConfig {
    pub fn load() -> Self {
        let content = fs::read_to_string("config.json").unwrap_or_else(|_| "{
            \"server\": {\"port\": 8080, \"tick_rate\": 20},
            \"game\": {\"world_width\": 2000, \"world_height\": 2000}
        }".to_string());
        
        serde_json::from_str(&content).expect("Failed to parse config")
    }
}
