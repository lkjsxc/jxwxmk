use serde::Deserialize;
use std::path::Path;
use config::{Config, File, Environment};
use thiserror::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub server: ServerSettings,
    pub database_url: String,
    pub game: GameSettings,
    pub security: SecuritySettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GameSettings {
    pub tick_rate: u32,
    pub max_players: usize,
    pub world_size: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SecuritySettings {
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub rate_limit_window: u64,
    pub rate_limit_max: usize,
}

impl ServerConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let mut builder = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::with_prefix("SERVER").prefix_separator("__"));
        
        if Path::new("config/production.toml").exists() {
            builder = builder.add_source(File::with_name("config/production"));
        }
        
        let config = builder.build()?;
        config.try_deserialize().map_err(ConfigError::from)
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Configuration file error: {0}")]
    FileError(#[from] config::ConfigError),
    #[error("Configuration deserialization error: {0}")]
    DeserializationError(String),
}

impl From<config::ConfigError> for ConfigError {
    fn from(err: config::ConfigError) -> Self {
        ConfigError::FileError(err)
    }
}

impl From<serde::de::value::Error> for ConfigError {
    fn from(err: serde::de::value::Error) -> Self {
        ConfigError::DeserializationError(err.to_string())
    }
}