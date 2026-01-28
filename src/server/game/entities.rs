use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityKind {
    Player,
    Resource,
    Mob,
    Structure,
    Npc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: Uuid,
    pub kind: EntityKind,
    pub subtype: String,
    pub x: f32,
    pub y: f32,
    pub hp: f32,
    pub max_hp: f32,
    pub level: u32,
    pub name: Option<String>,
    pub range: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub token: Uuid,
    pub username: String,
    pub x: f32,
    pub y: f32,
    pub hp: f32,
    pub max_hp: f32,
    pub hunger: f32,
    pub temperature: f32,
    pub level: u32,
    pub xp: u64,
    // Inventory and stats will be added later
}
