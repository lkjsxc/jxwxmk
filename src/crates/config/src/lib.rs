mod models;
mod loader;
mod tests;

pub use models::*;
pub use loader::{load_config, ConfigError};

pub fn init() {}