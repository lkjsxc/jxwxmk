use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingConfig {
    pub recipes: Vec<RecipeDef>,
}

impl Default for CraftingConfig {
    fn default() -> Self {
        Self { recipes: Vec::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeDef {
    pub id: String,
    pub station_tier: u32,
    pub inputs: Vec<RecipeInput>,
    pub output: RecipeOutput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeInput {
    pub item: String,
    pub count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeOutput {
    pub item: String,
    pub count: u32,
}
