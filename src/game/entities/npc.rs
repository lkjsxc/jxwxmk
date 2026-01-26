use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::game::entities::item::Item;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NpcType {
    Elder,
    Merchant,
    Guard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Npc {
    pub id: Uuid,
    pub n_type: NpcType,
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub health: f64,
    pub dialogue_index: u32,
    pub trade_inventory: Option<Vec<Item>>,
}

impl Npc {
    pub fn new(n_type: NpcType, name: &str, x: f64, y: f64) -> Self {
        let trade_inventory = match n_type {
            NpcType::Merchant => Some(vec![]), // Initialize with empty trade list
            _ => None,
        };

        Self {
            id: Uuid::new_v4(),
            n_type,
            name: name.to_string(),
            x,
            y,
            health: 100.0,
            dialogue_index: 0,
            trade_inventory,
        }
    }
}
