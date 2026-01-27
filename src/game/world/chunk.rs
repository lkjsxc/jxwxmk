use std::collections::HashMap;

use uuid::Uuid;

use super::coords::ChunkCoord;
use super::entities::{Mob, Npc, ResourceNode, Structure};

#[derive(Clone, Debug)]
pub struct Chunk {
    pub coord: ChunkCoord,
    pub biome_id: String,
    pub resources: HashMap<Uuid, ResourceNode>,
    pub mobs: HashMap<Uuid, Mob>,
    pub structures: HashMap<Uuid, Structure>,
    pub npcs: HashMap<Uuid, Npc>,
    pub settlement_id: Option<Uuid>,
    pub cooldowns: HashMap<String, f32>,
}

impl Chunk {
    pub fn new(coord: ChunkCoord, biome_id: String) -> Self {
        Self {
            coord,
            biome_id,
            resources: HashMap::new(),
            mobs: HashMap::new(),
            structures: HashMap::new(),
            npcs: HashMap::new(),
            settlement_id: None,
            cooldowns: HashMap::new(),
        }
    }
}
