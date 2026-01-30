use crate::chunk::*;
use crate::entity::*;
use crate::player::*;
use crate::settlement::*;
use config::WorldConfig;
use std::collections::{HashMap, HashSet};

pub type ChunkCoord = (i32, i32);
pub type PlayerId = uuid::Uuid;

pub struct World {
    pub seed: u64,
    pub chunk_size_wu: i32,
    pub chunks: HashMap<ChunkCoord, Chunk>,
    pub players: HashMap<PlayerId, PlayerState>,
    pub active_chunks: HashSet<ChunkCoord>,
    pub interest_sets: HashMap<PlayerId, HashSet<ChunkCoord>>,
    pub settlements: Vec<Settlement>,
}

impl World {
    pub fn new(config: &WorldConfig) -> Self {
        let seed = config.seed.parse::<u64>().unwrap_or(123456789);
        
        Self {
            seed,
            chunk_size_wu: config.chunk_size_wu,
            chunks: HashMap::new(),
            players: HashMap::new(),
            active_chunks: HashSet::new(),
            interest_sets: HashMap::new(),
            settlements: Vec::new(),
        }
    }

    pub fn get_or_create_chunk(&mut self, coord: ChunkCoord) -> &mut Chunk {
        self.chunks.entry(coord).or_insert_with(|| {
            Chunk::generate(coord, self.seed, self.chunk_size_wu)
        })
    }

    pub fn get_chunk(&self, coord: ChunkCoord) -> Option<&Chunk> {
        self.chunks.get(&coord)
    }

    pub fn world_to_chunk(&self, x: f64, y: f64) -> ChunkCoord {
        let cx = (x / self.chunk_size_wu as f64).floor() as i32;
        let cy = (y / self.chunk_size_wu as f64).floor() as i32;
        (cx, cy)
    }

    pub fn update_interest_set(&mut self, player_id: PlayerId, radius: i32) {
        if let Some(player) = self.players.get(&player_id) {
            if !player.spawned {
                return;
            }
            
            let (px, py) = (player.x, player.y);
            let (cx, cy) = self.world_to_chunk(px, py);
            
            let mut interest = HashSet::new();
            for dx in -radius..=radius {
                for dy in -radius..=radius {
                    interest.insert((cx + dx, cy + dy));
                }
            }
            
            self.interest_sets.insert(player_id, interest);
        }
    }

    pub fn activate_chunks(&mut self, view_radius: i32, sim_radius: i32) {
        let mut to_activate: HashSet<ChunkCoord> = HashSet::new();
        
        for (player_id, interest) in &self.interest_sets {
            if let Some(player) = self.players.get(player_id) {
                if player.spawned {
                    for &coord in interest {
                        to_activate.insert(coord);
                    }
                }
            }
        }
        
        self.active_chunks = to_activate;
    }

    pub fn add_player(&mut self, id: PlayerId, name: String) -> &mut PlayerState {
        self.players.entry(id).or_insert_with(|| PlayerState::new(id, name))
    }

    pub fn remove_player(&mut self, id: PlayerId) {
        self.players.remove(&id);
        self.interest_sets.remove(&id);
    }
}
