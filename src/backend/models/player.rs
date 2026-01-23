use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::item::Item;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub username: String,
    pub position: Position,
    pub health: i32,
    pub hunger: i32,
    pub inventory: Vec<Item>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}