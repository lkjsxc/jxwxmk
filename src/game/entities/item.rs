use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ItemType {
    Wood,
    Stone,
    Gold,
    Diamond,
    Berry,
    Meat,
    CookedMeat,
    WoodPickaxe,
    StonePickaxe,
    WoodWall,
    Door,
    Torch,
    Workbench,
    // Add more as needed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub kind: ItemType,
    pub amount: u32,
    pub max_stack: u32,
    pub level: u32,
    pub xp: f64,
}

impl Item {
    pub fn new(kind: ItemType, amount: u32) -> Self {
        Self {
            kind,
            amount,
            max_stack: u32::MAX,
            level: 1,
            xp: 0.0,
        }
    }
}
