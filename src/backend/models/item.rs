use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub item_type: ItemType,
    pub quantity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemType {
    Wood,
    Stone,
    Food,
    Tool,
    Weapon,
}