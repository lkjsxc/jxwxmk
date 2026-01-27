use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivalConfig {
    pub hunger_decay: f32,
    pub starve_damage: f32,
    pub heal_threshold: f32,
    pub heal_rate: f32,
    pub neutral_temp: f32,
    pub freeze_damage: f32,
    pub thirst_enabled: bool,
    pub thirst_decay: f32,
    pub thirst_damage: f32,
}

impl Default for SurvivalConfig {
    fn default() -> Self {
        Self {
            hunger_decay: 0.2,
            starve_damage: 1.0,
            heal_threshold: 80.0,
            heal_rate: 0.5,
            neutral_temp: 50.0,
            freeze_damage: 1.0,
            thirst_enabled: false,
            thirst_decay: 0.2,
            thirst_damage: 1.0,
        }
    }
}
