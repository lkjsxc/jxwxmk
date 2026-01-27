use std::collections::{HashMap, HashSet};

use super::chunk::ChunkCoord;
use super::entities::{BarrierCore, PlayerId, PlayerState};
use super::chunk::Chunk;

#[derive(Debug, Clone)]
pub struct Settlement {
    pub id: String,
    pub name: String,
    pub core: BarrierCore,
    pub spawn_x: f32,
    pub spawn_y: f32,
}

#[derive(Debug)]
pub struct World {
    pub seed: u64,
    pub chunks: HashMap<ChunkCoord, Chunk>,
    pub players: HashMap<PlayerId, PlayerState>,
    pub active_chunks: HashSet<ChunkCoord>,
    pub interest_sets: HashMap<PlayerId, HashSet<ChunkCoord>>,
    pub settlements: HashMap<String, Settlement>,
}

impl World {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            chunks: HashMap::new(),
            players: HashMap::new(),
            active_chunks: HashSet::new(),
            interest_sets: HashMap::new(),
            settlements: HashMap::new(),
        }
    }

    pub fn chunk_coord_from_pos(&self, x: f32, y: f32, chunk_size: f32) -> ChunkCoord {
        let cx = (x / chunk_size).floor() as i32;
        let cy = (y / chunk_size).floor() as i32;
        ChunkCoord::new(cx, cy)
    }

    pub fn upsert_player(&mut self, player: PlayerState) {
        self.players.insert(player.id, player);
    }

    pub fn get_player_mut(&mut self, player_id: &PlayerId) -> Option<&mut PlayerState> {
        self.players.get_mut(player_id)
    }

    pub fn get_player(&self, player_id: &PlayerId) -> Option<&PlayerState> {
        self.players.get(player_id)
    }

    pub fn is_in_safe_zone(&self, x: f32, y: f32) -> bool {
        self.settlements.values().any(|settlement| {
            let dx = x - settlement.core.x;
            let dy = y - settlement.core.y;
            dx * dx + dy * dy <= settlement.core.range().powi(2)
        })
    }
}
