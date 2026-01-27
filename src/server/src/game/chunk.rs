use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::entities::{Mob, Npc, ResourceNode, Structure};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkCoord {
    pub cx: i32,
    pub cy: i32,
}

impl ChunkCoord {
    pub fn new(cx: i32, cy: i32) -> Self {
        Self { cx, cy }
    }
}

#[derive(Debug, Clone)]
pub struct RespawnEntry {
    pub kind: RespawnKind,
    pub remaining: f32,
}

#[derive(Debug, Clone)]
pub enum RespawnKind {
    Resource { r_type: String, level: u32 },
    Mob { m_type: String, level: u32 },
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub coord: ChunkCoord,
    pub biome_id: String,
    pub resources: HashMap<String, ResourceNode>,
    pub mobs: HashMap<String, Mob>,
    pub structures: HashMap<String, Structure>,
    pub npcs: HashMap<String, Npc>,
    pub settlement_id: Option<String>,
    pub cooldowns: Vec<RespawnEntry>,
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
            cooldowns: Vec::new(),
        }
    }
}
