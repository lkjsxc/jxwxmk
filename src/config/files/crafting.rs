use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct CraftingConfig {
    pub recipes: Vec<RecipeDef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct RecipeDef {
    pub id: String,
    pub station_tier: u8,
    pub inputs: Vec<IngredientDef>,
    pub output: OutputDef,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct IngredientDef {
    pub item: String,
    pub count: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct OutputDef {
    pub item: String,
    pub count: u32,
}

impl Default for CraftingConfig {
    fn default() -> Self {
        Self {
            recipes: vec![
                RecipeDef {
                    id: "WoodPickaxe".to_string(),
                    station_tier: 0,
                    inputs: vec![IngredientDef {
                        item: "Wood".to_string(),
                        count: 10,
                    }],
                    output: OutputDef {
                        item: "WoodPickaxe".to_string(),
                        count: 1,
                    },
                },
                RecipeDef {
                    id: "StonePickaxe".to_string(),
                    station_tier: 0,
                    inputs: vec![
                        IngredientDef {
                            item: "Wood".to_string(),
                            count: 10,
                        },
                        IngredientDef {
                            item: "Stone".to_string(),
                            count: 10,
                        },
                    ],
                    output: OutputDef {
                        item: "StonePickaxe".to_string(),
                        count: 1,
                    },
                },
                RecipeDef {
                    id: "WoodWall".to_string(),
                    station_tier: 0,
                    inputs: vec![IngredientDef {
                        item: "Wood".to_string(),
                        count: 20,
                    }],
                    output: OutputDef {
                        item: "WoodWall".to_string(),
                        count: 1,
                    },
                },
                RecipeDef {
                    id: "Door".to_string(),
                    station_tier: 0,
                    inputs: vec![IngredientDef {
                        item: "Wood".to_string(),
                        count: 30,
                    }],
                    output: OutputDef {
                        item: "Door".to_string(),
                        count: 1,
                    },
                },
                RecipeDef {
                    id: "Torch".to_string(),
                    station_tier: 0,
                    inputs: vec![IngredientDef {
                        item: "Wood".to_string(),
                        count: 2,
                    }],
                    output: OutputDef {
                        item: "Torch".to_string(),
                        count: 1,
                    },
                },
                RecipeDef {
                    id: "Workbench".to_string(),
                    station_tier: 0,
                    inputs: vec![IngredientDef {
                        item: "Wood".to_string(),
                        count: 50,
                    }],
                    output: OutputDef {
                        item: "Workbench".to_string(),
                        count: 1,
                    },
                },
            ],
        }
    }
}

impl Default for RecipeDef {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            station_tier: 0,
            inputs: Vec::new(),
            output: OutputDef::default(),
        }
    }
}

impl Default for IngredientDef {
    fn default() -> Self {
        Self {
            item: "".to_string(),
            count: 0,
        }
    }
}

impl Default for OutputDef {
    fn default() -> Self {
        Self {
            item: "".to_string(),
            count: 0,
        }
    }
}
