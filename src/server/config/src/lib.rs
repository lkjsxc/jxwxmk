use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Validation error: {0}")]
    Validation(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServerConfig {
    pub version: i32,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_tick_rate")]
    pub tick_rate: u32,
    #[serde(default)]
    pub rate_limits: RateLimits,
}

fn default_port() -> u16 {
    8080
}

fn default_tick_rate() -> u32 {
    30
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct RateLimits {
    #[serde(default = "default_ws_messages_per_second")]
    pub ws_messages_per_second: u32,
    #[serde(default = "default_session_claim_per_minute")]
    pub session_claim_per_minute: u32,
}

fn default_ws_messages_per_second() -> u32 {
    60
}

fn default_session_claim_per_minute() -> u32 {
    10
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorldConfig {
    pub version: i32,
    #[serde(default = "default_chunk_size")]
    pub chunk_size: i32,
    #[serde(default = "default_simulation_radius")]
    pub simulation_radius: i32,
    #[serde(default = "default_view_radius")]
    pub view_radius: i32,
    #[serde(default)]
    pub seed: Option<u64>,
}

fn default_chunk_size() -> i32 {
    128
}

fn default_simulation_radius() -> i32 {
    3
}

fn default_view_radius() -> i32 {
    5
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BalanceConfig {
    pub version: i32,
    #[serde(default)]
    pub player: PlayerBalance,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct PlayerBalance {
    #[serde(default = "default_base_speed")]
    pub base_speed: f32,
    #[serde(default = "default_base_hp")]
    pub base_hp: f32,
}

fn default_base_speed() -> f32 {
    5.0
}

fn default_base_hp() -> f32 {
    30.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SurvivalConfig {
    pub version: i32,
    #[serde(default)]
    pub hunger_enabled: bool,
    #[serde(default = "default_hunger_decay_per_second")]
    pub hunger_decay_per_second: f32,
    #[serde(default)]
    pub temperature_enabled: bool,
    #[serde(default = "default_temperature_convergence_rate")]
    pub temperature_convergence_rate: f32,
    #[serde(default)]
    pub thirst_enabled: bool,
}

fn default_hunger_decay_per_second() -> f32 {
    0.5
}

fn default_temperature_convergence_rate() -> f32 {
    0.1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CraftingConfig {
    pub version: i32,
    #[serde(default)]
    pub recipes: HashMap<String, Recipe>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Recipe {
    pub ingredients: Vec<Ingredient>,
    pub output: String,
    pub count: i32,
    #[serde(default)]
    pub station_required: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ingredient {
    pub item: String,
    pub count: i32,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub world: WorldConfig,
    pub balance: BalanceConfig,
    pub survival: SurvivalConfig,
    pub crafting: CraftingConfig,
}

impl Config {
    pub fn load_from_dir(path: &Path) -> Result<Self, ConfigError> {
        let server = load_config_file(&path.join("server.json"))?;
        let world = load_config_file(&path.join("world.json"))?;
        let balance = load_config_file(&path.join("balance.json"))?;
        let survival = load_config_file(&path.join("survival.json"))?;
        let crafting = load_config_file(&path.join("crafting.json"))?;

        Ok(Config {
            server,
            world,
            balance,
            survival,
            crafting,
        })
    }

    pub fn load_defaults() -> Self {
        Config {
            server: ServerConfig {
                version: 1,
                port: default_port(),
                tick_rate: default_tick_rate(),
                rate_limits: RateLimits::default(),
            },
            world: WorldConfig {
                version: 1,
                chunk_size: default_chunk_size(),
                simulation_radius: default_simulation_radius(),
                view_radius: default_view_radius(),
                seed: None,
            },
            balance: BalanceConfig {
                version: 1,
                player: PlayerBalance::default(),
            },
            survival: SurvivalConfig {
                version: 1,
                hunger_enabled: true,
                hunger_decay_per_second: default_hunger_decay_per_second(),
                temperature_enabled: true,
                temperature_convergence_rate: default_temperature_convergence_rate(),
                thirst_enabled: false,
            },
            crafting: CraftingConfig {
                version: 1,
                recipes: HashMap::new(),
            },
        }
    }
}

fn load_config_file<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, ConfigError> {
    if !path.exists() {
        return Err(ConfigError::Validation(format!(
            "Config file not found: {}",
            path.display()
        )));
    }
    let content = fs::read_to_string(path)?;
    let config: T = serde_json::from_str(&content)?;
    Ok(config)
}
