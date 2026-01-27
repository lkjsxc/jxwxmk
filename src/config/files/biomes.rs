use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct BiomesConfig {
    pub biomes: Vec<BiomeDef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct BiomeDef {
    pub id: String,
    pub hunger_modifier: f32,
    pub temperature_modifier: f32,
}

impl Default for BiomesConfig {
    fn default() -> Self {
        Self {
            biomes: vec![
                BiomeDef {
                    id: "forest".to_string(),
                    hunger_modifier: 1.0,
                    temperature_modifier: 0.0,
                },
                BiomeDef {
                    id: "tundra".to_string(),
                    hunger_modifier: 1.1,
                    temperature_modifier: -15.0,
                },
            ],
        }
    }
}

impl Default for BiomeDef {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            hunger_modifier: 1.0,
            temperature_modifier: 0.0,
        }
    }
}
