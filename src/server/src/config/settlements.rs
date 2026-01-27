use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementsConfig {
    pub core_base_range: f32,
    pub core_level_multiplier: f32,
    pub default_core_level: u32,
    pub settlement_names: Vec<String>,
}

impl Default for SettlementsConfig {
    fn default() -> Self {
        Self {
            core_base_range: 120.0,
            core_level_multiplier: 40.0,
            default_core_level: 2,
            settlement_names: vec!["Elder Hollow".to_string()],
        }
    }
}
