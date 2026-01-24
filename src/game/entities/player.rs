use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Inventory {
    pub wood: i32,
    pub stone: i32,
    pub food: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub username: String,
    pub x: f64,
    pub y: f64,
    pub health: f64,
    pub hunger: f64,
    pub cold: f64,
    pub inventory: Inventory,
}

impl Player {
    pub fn new(id: Uuid, username: String, x: f64, y: f64) -> Self {
        Self {
            id,
            username,
            x,
            y,
            health: 100.0,
            hunger: 100.0,
            cold: 0.0,
            inventory: Inventory::default(),
        }
    }
}
