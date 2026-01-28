use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use protocol::{EntitySnapshot, Vec2};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entity {
    pub id: String,
    pub kind: String,
    pub subtype: String,
    pub pos: Vec2,
    pub hp: Option<f32>,
    pub max_hp: Option<f32>,
    pub level: Option<u32>,
    pub name: Option<String>,
}

impl Entity {
    pub fn to_snapshot(&self) -> EntitySnapshot {
        EntitySnapshot {
            id: self.id.clone(),
            kind: self.kind.clone(),
            subtype: self.subtype.clone(),
            x: self.pos.x,
            y: self.pos.y,
            hp: self.hp,
            max_hp: self.max_hp,
            level: self.level,
            name: self.name.clone(),
            range: None,
        }
    }
}

pub struct Chunk {
    pub coord: (i32, i32),
    pub biome: String,
    pub entities: HashMap<String, Entity>,
}

pub struct World {
    pub chunks: HashMap<(i32, i32), Chunk>,
    pub players: HashMap<Uuid, String>, // Player UUID to Entity ID mapping
}

impl World {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            players: HashMap::new(),
        }
    }

    pub fn get_chunk_mut(&mut self, coord: (i32, i32)) -> &mut Chunk {
        self.chunks.entry(coord).or_insert_with(|| Chunk {
            coord,
            biome: "forest".to_string(),
            entities: HashMap::new(),
        })
    }
}
