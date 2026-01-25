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
    pub interact_range: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MechanicsConfig {
    pub hunger_decay: f64,
    pub cold_decay: f64,
    pub heal_rate: f64,
    pub starve_dmg: f64,
    pub freeze_dmg: f64,
    pub food_value: f64,
    pub attack_cooldown: u64,
    pub interact_cooldown: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub game: GameConfig,
    pub mechanics: MechanicsConfig,
}

impl AppConfig {
    pub fn load() -> Self {
        let content = fs::read_to_string("config.json").expect("config.json missing");
        serde_json::from_str(&content).expect("Failed to parse config")
    }
}