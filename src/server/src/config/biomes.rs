use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomesConfig {
    pub biomes: Vec<BiomeDef>,
}

impl Default for BiomesConfig {
    fn default() -> Self {
        Self { biomes: Vec::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeDef {
    pub id: String,
    pub hunger_modifier: f32,
    pub temperature_modifier: f32,
}
