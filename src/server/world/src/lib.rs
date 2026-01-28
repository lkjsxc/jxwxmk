use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

pub mod biome;
pub mod spawning;

pub type ChunkCoord = (i32, i32);
pub type PlayerId = Uuid;
pub type EntityId = String;

#[derive(Debug, Clone)]
pub struct World {
    pub seed: u64,
    pub chunks: HashMap<ChunkCoord, Chunk>,
    pub players: HashMap<PlayerId, PlayerState>,
    pub active_chunks: HashSet<ChunkCoord>,
    pub interest_sets: HashMap<PlayerId, HashSet<ChunkCoord>>,
}

impl World {
    pub fn new(seed: u64) -> Self {
        World {
            seed,
            chunks: HashMap::new(),
            players: HashMap::new(),
            active_chunks: HashSet::new(),
            interest_sets: HashMap::new(),
        }
    }

    pub fn get_or_create_chunk(&mut self, coord: ChunkCoord) -> &mut Chunk {
        self.chunks.entry(coord).or_insert_with(|| Chunk::new(coord))
    }

    pub fn update_interest_set(
        &mut self,
        player_id: PlayerId,
        player_pos: (f32, f32),
        view_radius: i32,
    ) -> (Vec<ChunkCoord>, Vec<ChunkCoord>) {
        let player_chunk = pos_to_chunk(player_pos);
        let mut new_interest = HashSet::new();

        for dx in -view_radius..=view_radius {
            for dy in -view_radius..=view_radius {
                new_interest.insert((player_chunk.0 + dx, player_chunk.1 + dy));
            }
        }

        let old_interest = self.interest_sets.get(&player_id).cloned().unwrap_or_default();
        let added: Vec<_> = new_interest.difference(&old_interest).copied().collect();
        let removed: Vec<_> = old_interest.difference(&new_interest).copied().collect();

        self.interest_sets.insert(player_id, new_interest);
        (added, removed)
    }
}

pub fn pos_to_chunk(pos: (f32, f32)) -> ChunkCoord {
    const CHUNK_SIZE: f32 = 128.0;
    (
        (pos.0 / CHUNK_SIZE).floor() as i32,
        (pos.1 / CHUNK_SIZE).floor() as i32,
    )
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub coord: ChunkCoord,
    pub biome_id: String,
    pub resources: HashMap<EntityId, Resource>,
    pub mobs: HashMap<EntityId, Mob>,
    pub structures: HashMap<EntityId, Structure>,
    pub npcs: HashMap<EntityId, Npc>,
    pub settlement_id: Option<Uuid>,
    pub cooldowns: HashMap<String, f32>,
    pub dirty: bool,
}

impl Chunk {
    pub fn new(coord: ChunkCoord) -> Self {
        Chunk {
            coord,
            biome_id: "forest".to_string(),
            resources: HashMap::new(),
            mobs: HashMap::new(),
            structures: HashMap::new(),
            npcs: HashMap::new(),
            settlement_id: None,
            cooldowns: HashMap::new(),
            dirty: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub id: EntityId,
    pub subtype: String,
    pub x: f32,
    pub y: f32,
    pub hp: f32,
    pub max_hp: f32,
    pub level: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mob {
    pub id: EntityId,
    pub subtype: String,
    pub x: f32,
    pub y: f32,
    pub hp: f32,
    pub max_hp: f32,
    pub level: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Structure {
    pub id: EntityId,
    pub subtype: String,
    pub x: f32,
    pub y: f32,
    pub hp: f32,
    pub max_hp: f32,
    pub level: i32,
    pub owner_id: Option<PlayerId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Npc {
    pub id: EntityId,
    pub subtype: String,
    pub x: f32,
    pub y: f32,
    pub name: String,
    pub level: i32,
}

#[derive(Debug, Clone)]
pub struct PlayerState {
    pub id: PlayerId,
    pub name: String,
    pub spawned: bool,
    pub x: f32,
    pub y: f32,
    pub vitals: Vitals,
    pub inventory: Vec<Option<InventorySlot>>,
    pub active_slot: usize,
    pub level: i32,
    pub xp: i64,
    pub stats: PlayerStats,
    pub quests: Vec<Quest>,
    pub achievements: Vec<String>,
    pub settlement_id: Option<Uuid>,
}

impl PlayerState {
    pub fn new(id: PlayerId, name: String) -> Self {
        PlayerState {
            id,
            name,
            spawned: false,
            x: 0.0,
            y: 0.0,
            vitals: Vitals {
                hp: 30.0,
                max_hp: 30.0,
                hunger: 80.0,
                max_hunger: 100.0,
                temperature: 50.0,
                max_temperature: 100.0,
            },
            inventory: vec![None; 30],
            active_slot: 0,
            level: 1,
            xp: 0,
            stats: PlayerStats {
                steps: 0,
                kills: 0,
                crafts: 0,
                gathers: 0,
                deaths: 0,
            },
            quests: Vec::new(),
            achievements: Vec::new(),
            settlement_id: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Vitals {
    pub hp: f32,
    pub max_hp: f32,
    pub hunger: f32,
    pub max_hunger: f32,
    pub temperature: f32,
    pub max_temperature: f32,
}

#[derive(Debug, Clone)]
pub struct InventorySlot {
    pub item: String,
    pub count: i32,
}

#[derive(Debug, Clone)]
pub struct PlayerStats {
    pub steps: i64,
    pub kills: i64,
    pub crafts: i64,
    pub gathers: i64,
    pub deaths: i64,
}

#[derive(Debug, Clone)]
pub struct Quest {
    pub id: String,
    pub name: String,
    pub state: QuestState,
    pub objectives: Vec<Objective>,
}

#[derive(Debug, Clone)]
pub enum QuestState {
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug, Clone)]
pub struct Objective {
    pub description: String,
    pub completed: bool,
}

#[derive(Debug, Clone)]
pub struct Settlement {
    pub id: Uuid,
    pub name: String,
    pub core_level: i32,
    pub core_integrity: f32,
    pub core_x: f32,
    pub core_y: f32,
    pub safe_zone_radius: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityDelta {
    pub id: EntityId,
    pub kind: EntityKind,
    pub subtype: String,
    pub x: f32,
    pub y: f32,
    pub hp: Option<f32>,
    pub max_hp: Option<f32>,
    pub level: Option<i32>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityKind {
    Resource,
    Mob,
    Structure,
    Npc,
    Player,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkDelta {
    pub coord: ChunkCoord,
    pub updates: Vec<EntityDelta>,
    pub removes: Vec<EntityId>,
}

impl Chunk {
    pub fn build_delta(&self, _previous: Option<&Self>) -> ChunkDelta {
        let mut updates = Vec::new();
        let mut removes = Vec::new();

        for resource in self.resources.values() {
            updates.push(EntityDelta {
                id: resource.id.clone(),
                kind: EntityKind::Resource,
                subtype: resource.subtype.clone(),
                x: resource.x,
                y: resource.y,
                hp: Some(resource.hp),
                max_hp: Some(resource.max_hp),
                level: Some(resource.level),
                name: None,
            });
        }

        for mob in self.mobs.values() {
            updates.push(EntityDelta {
                id: mob.id.clone(),
                kind: EntityKind::Mob,
                subtype: mob.subtype.clone(),
                x: mob.x,
                y: mob.y,
                hp: Some(mob.hp),
                max_hp: Some(mob.max_hp),
                level: Some(mob.level),
                name: None,
            });
        }

        for structure in self.structures.values() {
            updates.push(EntityDelta {
                id: structure.id.clone(),
                kind: EntityKind::Structure,
                subtype: structure.subtype.clone(),
                x: structure.x,
                y: structure.y,
                hp: Some(structure.hp),
                max_hp: Some(structure.max_hp),
                level: Some(structure.level),
                name: None,
            });
        }

        for npc in self.npcs.values() {
            updates.push(EntityDelta {
                id: npc.id.clone(),
                kind: EntityKind::Npc,
                subtype: npc.subtype.clone(),
                x: npc.x,
                y: npc.y,
                hp: None,
                max_hp: None,
                level: Some(npc.level),
                name: Some(npc.name.clone()),
            });
        }

        ChunkDelta {
            coord: self.coord,
            updates,
            removes,
        }
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }
}

impl World {
    pub fn get_player_delta(
        &self,
        player_id: PlayerId,
    ) -> Vec<ChunkDelta> {
        let mut deltas = Vec::new();

        if let Some(interest_set) = self.interest_sets.get(&player_id) {
            for coord in interest_set {
                if let Some(chunk) = self.chunks.get(coord) {
                    deltas.push(chunk.build_delta(None));
                }
            }
        }

        deltas
    }

    pub fn get_dirty_chunks(&self) -> Vec<ChunkCoord> {
        self.chunks
            .iter()
            .filter(|(_, chunk)| chunk.dirty)
            .map(|(coord, _)| *coord)
            .collect()
    }

    pub fn clear_dirty_chunks(&mut self) {
        for chunk in self.chunks.values_mut() {
            chunk.clear_dirty();
        }
    }

    pub fn generate_settlement(
        &mut self,
        x: f32,
        y: f32,
        name: String,
    ) -> Settlement {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        let settlement = Settlement {
            id: Uuid::new_v4(),
            name,
            core_level: 1,
            core_integrity: 100.0,
            core_x: x,
            core_y: y,
            safe_zone_radius: 50.0,
        };

        let chunk_coord = pos_to_chunk((x, y));
        let chunk = self.get_or_create_chunk(chunk_coord);
        chunk.settlement_id = Some(settlement.id);
        chunk.mark_dirty();

        let npc_id = format!("npc_{}", rng.gen::<u32>());
        chunk.npcs.insert(
            npc_id.clone(),
            Npc {
                id: npc_id,
                subtype: "trader".to_string(),
                x: x + 5.0,
                y: y,
                name: "Trader".to_string(),
                level: 1,
            },
        );

        let structure_id = format!("structure_{}", rng.gen::<u32>());
        chunk.structures.insert(
            structure_id.clone(),
            Structure {
                id: structure_id,
                subtype: "barrier_core".to_string(),
                x,
                y,
                hp: 100.0,
                max_hp: 100.0,
                level: 1,
                owner_id: None,
            },
        );

        settlement
    }

    pub fn is_in_safe_zone(&self, x: f32, y: f32) -> bool {
        for chunk in self.chunks.values() {
            if let Some(settlement_id) = chunk.settlement_id {
                for settlement in self.get_settlements() {
                    if settlement.id == settlement_id {
                        let dx = x - settlement.core_x;
                        let dy = y - settlement.core_y;
                        let distance = (dx * dx + dy * dy).sqrt();
                        if distance <= settlement.safe_zone_radius {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    fn get_settlements(&self) -> Vec<Settlement> {
        let mut settlements = Vec::new();
        for chunk in self.chunks.values() {
            if let Some(settlement_id) = chunk.settlement_id {
                if !settlements.iter().any(|s: &Settlement| s.id == settlement_id) {
                    let settlement = Settlement {
                        id: settlement_id,
                        name: "Settlement".to_string(),
                        core_level: 1,
                        core_integrity: 100.0,
                        core_x: chunk.coord.0 as f32 * 128.0 + 64.0,
                        core_y: chunk.coord.1 as f32 * 128.0 + 64.0,
                        safe_zone_radius: 50.0,
                    };
                    settlements.push(settlement);
                }
            }
        }
        settlements
    }
}
