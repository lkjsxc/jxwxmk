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
}

impl Item {
    pub fn new(kind: ItemType, amount: u32) -> Self {
        let max_stack = match kind {
            ItemType::WoodPickaxe | ItemType::StonePickaxe => 1,
            _ => 64,
        };
        Self {
            kind,
            amount,
            max_stack,
        }
    }
}
