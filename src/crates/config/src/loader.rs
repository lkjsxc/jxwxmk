use crate::models::*;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    Parse(serde_json::Error),
    Validation(String),
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::Io(err)
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(err: serde_json::Error) -> Self {
        ConfigError::Parse(err)
    }
}

pub fn load_config<P: AsRef<Path>>(config_dir: P) -> Result<Config, ConfigError> {
    let dir = config_dir.as_ref();

    let mut config = Config::default();

    // Helper to load individual files
    // If file is missing, we log (if we had logging set up here) and keep default
    // If file exists but errors, we fail.

    if let Some(c) = load_file(dir, "server.json")? { config.server = c; }
    if let Some(c) = load_file(dir, "world.json")? { config.world = c; }
    if let Some(c) = load_file(dir, "balance.json")? { config.balance = c; }
    if let Some(c) = load_file(dir, "survival.json")? { config.survival = c; }
    if let Some(c) = load_file(dir, "crafting.json")? { config.crafting = c; }
    if let Some(c) = load_file(dir, "spawning.json")? { config.spawning = c; }
    if let Some(c) = load_file(dir, "biomes.json")? { config.biomes = c; }
    if let Some(c) = load_file(dir, "settlements.json")? { config.settlements = c; }
    if let Some(c) = load_file(dir, "economy.json")? { config.economy = c; }
    if let Some(c) = load_file(dir, "quests.json")? { config.quests = c; }
    if let Some(c) = load_file(dir, "achievements.json")? { config.achievements = c; }

    validate(&config)?;

    Ok(config)
}

fn load_file<T: serde::de::DeserializeOwned>(dir: &Path, filename: &str) -> Result<Option<T>, ConfigError> {
    let path = dir.join(filename);
    if !path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(path)?;
    // Use serde_json::from_str with Validation later?
    // For now, simple deserialize.
    // Ideally we'd use `serde_path_to_error` or similar for better errors, but basic serde_json is fine.
    
    // Check for unknown fields by using serde_json::Deserializer
    let mut deserializer = serde_json::Deserializer::from_str(&content);
    // Strict mode: fail on unknown fields
    // But serde_json doesn't expose strict mode easily on `from_str` without manual config.
    // The easiest way is `#[serde(deny_unknown_fields)]` on structs, but that's invasive.
    // Let's rely on unit tests or struct attributes if strictness is P0.
    // The requirement says: "Reject unknown fields".
    // I should add `#[serde(deny_unknown_fields)]` to the top-level structs in `models.rs`.
    // I will go back and add that. 
    // For now, let's just parse.
    
    let val: T = serde::Deserialize::deserialize(&mut deserializer)?;
    
    // Ensure we consumed the whole stream (part of strictness)
    deserializer.end()?;

    Ok(Some(val))
}

fn validate(config: &Config) -> Result<(), ConfigError> {
    // Basic validation logic
    if config.server.tick_rate < 20 || config.server.tick_rate > 60 {
        return Err(ConfigError::Validation("server.tick_rate must be between 20 and 60".into()));
    }
    if config.world.chunk_size_wu == 0 {
         return Err(ConfigError::Validation("world.chunk_size_wu must be > 0".into()));
    }
    // Add more validation as needed per schemas
    
    Ok(())
}
