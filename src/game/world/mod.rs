use std::collections::{HashMap, HashSet};

use rand::{Rng, SeedableRng};
use uuid::Uuid;

use crate::config::Config;

use self::chunk::Chunk;
use self::coords::ChunkCoord;
use self::entities::{Mob, Npc, PlayerState, ResourceNode, Structure};
use self::settlement::Settlement;

pub mod chunk;
pub mod coords;
pub mod entities;
pub mod settlement;

#[derive(Debug)]
pub struct World {
    pub seed: u64,
    pub chunks: HashMap<ChunkCoord, Chunk>,
    pub players: HashMap<Uuid, PlayerState>,
    pub active_chunks: HashSet<ChunkCoord>,
    pub interest_sets: HashMap<Uuid, HashSet<ChunkCoord>>,
    pub settlements: HashMap<Uuid, Settlement>,
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

    pub fn chunk_coord(&self, x: f32, y: f32, chunk_size: f32) -> ChunkCoord {
        ChunkCoord::new((x / chunk_size).floor() as i32, (y / chunk_size).floor() as i32)
    }

    pub fn ensure_chunk(&mut self, coord: ChunkCoord, config: &Config) -> &mut Chunk {
        if !self.chunks.contains_key(&coord) {
            let biome = config
                .biomes
                .biomes
                .first()
                .map(|b| b.id.clone())
                .unwrap_or_else(|| "forest".to_string());
            let mut chunk = Chunk::new(coord, biome);
            let mut rng = rand::rngs::StdRng::seed_from_u64(
                self.seed ^ ((coord.x as u64) << 32) ^ coord.y as u64,
            );
            let max_resources = config.spawning.max_resources_per_chunk.min(6);
            for _ in 0..max_resources {
                let x = rng.gen_range(-config.world.chunk_size / 2.0..config.world.chunk_size / 2.0);
                let y = rng.gen_range(-config.world.chunk_size / 2.0..config.world.chunk_size / 2.0);
                chunk
                    .resources
                    .insert(Uuid::new_v4(), ResourceNode::new("tree", x, y));
            }
            let max_mobs = config.spawning.max_mobs_per_chunk.min(2);
            for _ in 0..max_mobs {
                let x = rng.gen_range(-config.world.chunk_size / 2.0..config.world.chunk_size / 2.0);
                let y = rng.gen_range(-config.world.chunk_size / 2.0..config.world.chunk_size / 2.0);
                chunk.mobs.insert(Uuid::new_v4(), Mob::new("wolf", x, y));
            }
            self.chunks.insert(coord, chunk);
        }
        self.chunks.get_mut(&coord).expect("chunk exists")
    }

    pub fn update_player_chunk(&mut self, player_id: Uuid, chunk_size: f32) {
        if let Some(player) = self.players.get_mut(&player_id) {
            let coord = self.chunk_coord(player.x, player.y, chunk_size);
            player.chunk_x = coord.x;
            player.chunk_y = coord.y;
        }
    }

    pub fn interest_set(&self, player: &PlayerState, view_radius: i32) -> HashSet<ChunkCoord> {
        let mut set = HashSet::new();
        for dx in -view_radius..=view_radius {
            for dy in -view_radius..=view_radius {
                set.insert(ChunkCoord::new(player.chunk_x + dx, player.chunk_y + dy));
            }
        }
        set
    }

    pub fn is_in_safe_zone(&self, x: f32, y: f32) -> bool {
        self.settlements.values().any(|settlement| settlement.contains(x, y))
    }

    pub fn spawn_settlement(&mut self, config: &Config) -> Settlement {
        let mut rng = rand::thread_rng();
        let settlement_id = Uuid::new_v4();
        let name = config
            .settlements
            .settlement_names
            .first()
            .cloned()
            .unwrap_or_else(|| "Settlement".to_string());
        let core_level = config.settlements.default_core_level;
        let safe_radius = config.settlements.core_base_range
            + (core_level.saturating_sub(1) as f32) * config.settlements.core_level_multiplier;

        let settlement = Settlement {
            id: settlement_id,
            name,
            center_x: 0.0,
            center_y: 0.0,
            core_level,
            safe_radius,
            spawn_x: 8.0,
            spawn_y: 8.0,
        };

        let coord = ChunkCoord::new(0, 0);
        let chunk = self.ensure_chunk(coord, config);
        chunk.settlement_id = Some(settlement_id);
        chunk.structures.insert(
            Uuid::new_v4(),
            Structure::new("BarrierCore", settlement.center_x, settlement.center_y),
        );
        chunk.npcs.insert(
            Uuid::new_v4(),
            Npc::new("trader", "Trader Lina", 4.0, 6.0),
        );
        if chunk.resources.len() < config.spawning.max_resources_per_chunk {
            for _ in 0..3 {
                let x = rng.gen_range(-20.0..20.0);
                let y = rng.gen_range(-20.0..20.0);
                chunk
                    .resources
                    .insert(Uuid::new_v4(), ResourceNode::new("tree", x, y));
            }
        }

        self.settlements.insert(settlement_id, settlement.clone());
        settlement
    }

    pub fn nearby_resource(&self, x: f32, y: f32, radius: f32) -> Option<(ChunkCoord, Uuid)> {
        let mut best: Option<(ChunkCoord, Uuid, f32)> = None;
        for (coord, chunk) in &self.chunks {
            for (id, node) in &chunk.resources {
                let dx = node.x - x;
                let dy = node.y - y;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist <= radius {
                    match best {
                        None => best = Some((*coord, *id, dist)),
                        Some((_, best_id, best_dist)) => {
                            if dist < best_dist || (dist == best_dist && id.as_bytes() < best_id.as_bytes()) {
                                best = Some((*coord, *id, dist));
                            }
                        }
                    }
                }
            }
        }
        best.map(|(coord, id, _)| (coord, id))
    }

    pub fn nearby_mob(&self, x: f32, y: f32, radius: f32) -> Option<(ChunkCoord, Uuid)> {
        let mut best: Option<(ChunkCoord, Uuid, f32)> = None;
        for (coord, chunk) in &self.chunks {
            for (id, mob) in &chunk.mobs {
                let dx = mob.x - x;
                let dy = mob.y - y;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist <= radius {
                    match best {
                        None => best = Some((*coord, *id, dist)),
                        Some((_, best_id, best_dist)) => {
                            if dist < best_dist || (dist == best_dist && id.as_bytes() < best_id.as_bytes()) {
                                best = Some((*coord, *id, dist));
                            }
                        }
                    }
                }
            }
        }
        best.map(|(coord, id, _)| (coord, id))
    }

    pub fn add_mob(&mut self, coord: ChunkCoord, mob: Mob, config: &Config) {
        let chunk = self.ensure_chunk(coord, config);
        chunk.mobs.insert(Uuid::new_v4(), mob);
    }
}
