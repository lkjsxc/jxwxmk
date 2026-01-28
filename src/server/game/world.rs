use std::collections::HashMap;
use uuid::Uuid;
// use serde::{Deserialize, Serialize}; // Unused for now

#[derive(Debug)]
pub struct World {
    pub seed: u32,
    pub chunks: HashMap<(i32, i32), Chunk>,
    pub players: HashMap<Uuid, PlayerState>,
    // active_chunks: bounded set of loaded chunks (TODO)
}

impl World {
    pub fn new() -> Self {
        Self {
            seed: 0, // TODO: Load from config
            chunks: HashMap::new(),
            players: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Chunk {
    pub coord: (i32, i32),
    pub biome_id: String,
    // Entities
    pub resources: HashMap<Uuid, Resource>,
    pub mobs: HashMap<Uuid, Mob>,
    pub structures: HashMap<Uuid, Structure>,
    pub npcs: HashMap<Uuid, Npc>,
}

#[derive(Debug)]
pub struct PlayerState {
    pub id: Uuid,
    pub token: Uuid,
    pub username: String,
    pub x: f64,
    pub y: f64,
    pub health: f64,
    pub max_health: f64,
    pub spawned: bool,
    // TODO: Inventory, stats, etc.
}

#[derive(Debug)]
pub struct Resource {
    pub id: Uuid,
    pub kind: String,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct Mob {
    pub id: Uuid,
    pub kind: String,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct Structure {
    pub id: Uuid,
    pub kind: String,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct Npc {
    pub id: Uuid,
    pub kind: String,
    pub x: f64,
    pub y: f64,
}
