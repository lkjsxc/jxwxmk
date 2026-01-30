use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub mod schemas;
pub use schemas::*;

#[derive(Debug, Clone, thiserror::Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(String),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Validation error: {0}")]
    Validation(String),
}

#[derive(Debug, Clone)]
pub struct GameConfig {
    pub server: ServerConfig,
    pub world: WorldConfig,
    pub balance: BalanceConfig,
    pub survival: SurvivalConfig,
    pub crafting: CraftingConfig,
    pub spawning: SpawningConfig,
    pub biomes: BiomesConfig,
    pub settlements: SettlementsConfig,
    pub economy: EconomyConfig,
    pub quests: QuestsConfig,
    pub achievements: AchievementsConfig,
}

impl GameConfig {
    pub fn load_from_dir<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        
        Ok(Self {
            server: load_config_file(path, "server.json")?,
            world: load_config_file(path, "world.json")?,
            balance: load_config_file(path, "balance.json")?,
            survival: load_config_file(path, "survival.json")?,
            crafting: load_config_file(path, "crafting.json")?,
            spawning: load_config_file(path, "spawning.json")?,
            biomes: load_config_file(path, "biomes.json")?,
            settlements: load_config_file(path, "settlements.json")?,
            economy: load_config_file(path, "economy.json")?,
            quests: load_config_file(path, "quests.json")?,
            achievements: load_config_file(path, "achievements.json")?,
        })
    }
}

fn load_config_file<P: AsRef<Path>, T: for<'de> Deserialize<'de> + Default>(
    dir: P,
    filename: &str,
) -> Result<T, ConfigError> {
    let path = dir.as_ref().join(filename);
    
    if !path.exists() {
        log::warn!("Config file {} not found, using defaults", filename);
        return Ok(T::default());
    }
    
    let content = fs::read_to_string(&path)
        .map_err(|e| ConfigError::Io(format!("Failed to read {}: {}", filename, e)))?;
    
    let config: T = serde_json::from_str(&content)
        .map_err(|e| ConfigError::Parse(format!("Failed to parse {}: {}", filename, e)))?;
    
    Ok(config)
}
