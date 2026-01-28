use std::collections::HashMap;
use uuid::Uuid;
use super::entities::{Entity, Player};

pub type ChunkCoord = (i32, i32);

#[derive(Debug, Default)]
pub struct Chunk {
    pub coord: ChunkCoord,
    pub biome: String,
    pub resources: HashMap<Uuid, Entity>,
    pub mobs: HashMap<Uuid, Entity>,
    pub structures: HashMap<Uuid, Entity>,
    pub npcs: HashMap<Uuid, Entity>,
    pub settlement_id: Option<Uuid>,
}

pub struct World {
    pub seed: u32,
    pub chunks: HashMap<ChunkCoord, Chunk>,
    pub players: HashMap<Uuid, Player>,
    pub active_chunks: Vec<ChunkCoord>,
}

impl World {
    pub fn new(seed: u32) -> Self {
        Self {
            seed,
            chunks: HashMap::new(),
            players: HashMap::new(),
            active_chunks: Vec::new(),
        }
    }
}
