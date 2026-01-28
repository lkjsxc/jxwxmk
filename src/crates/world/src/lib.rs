use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::{HashMap, HashSet};
use config::Config;

// Re-export common types from protocol to avoid duplication and version drift
// Or define them here and map? Module map suggests world -> config -> protocol is allowed.
// But world -> protocol is NOT allowed per module map: "world -> config".
// Actually module map says: "world -> config", and "config -> protocol".
// So world cannot depend on protocol.
// We must define domain types here and map them in `game` or `net`.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

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
    pub id: Uuid, // Internal UUID
    pub kind: EntityKind,
    pub subtype: String, // "tree", "wolf", "wall"
    pub pos: Vec2,
    pub hp: f32,
    pub max_hp: f32,
    pub name: Option<String>,
    pub owner_id: Option<Uuid>, // For structures
    pub target_pos: Option<Vec2>, // For AI
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub coord: ChunkCoord,
    pub biome: String,
    pub resources: HashMap<Uuid, Entity>,
    pub mobs: HashMap<Uuid, Entity>,
    pub structures: HashMap<Uuid, Entity>,
    pub npcs: HashMap<Uuid, Entity>,
    pub players: HashSet<Uuid>, // Just references to world.players
    #[serde(skip)]
    pub respawn_queue: Vec<(std::time::Instant, Entity)>,
    pub dirty: bool,
}

impl Chunk {
    pub fn new(coord: ChunkCoord, biome: String) -> Self {
        Self {
            coord,
            biome,
            resources: HashMap::new(),
            mobs: HashMap::new(),
            structures: HashMap::new(),
            npcs: HashMap::new(),
            players: HashSet::new(),
            respawn_queue: Vec::new(),
            dirty: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventorySlot {
    pub item_id: String,
    pub count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveQuest {
    pub id: String,
    pub status: QuestStatus,
    pub objectives: Vec<QuestObjectiveProgress>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QuestStatus {
    InProgress,
    ReadyToTurnIn,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestObjectiveProgress {
    pub objective_type: String, // "kill", "gather", etc.
    pub target: String,         // "wolf", "wood", etc.
    pub required: u32,
    pub current: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub id: Uuid,
    pub token: Uuid,
    pub name: String,
    pub level: u32,
    pub xp: u64,
    pub pos: Vec2,
    pub chunk: ChunkCoord,
    pub hp: f32,
    pub max_hp: f32,
    pub hunger: f32,
    pub thirst: f32,
    pub temp: f32,
    pub inventory: Vec<Option<InventorySlot>>,
    pub active_slot: usize,
    pub stats: HashMap<String, f32>, // Generic stats for achievements
    pub unlocked_achievements: HashSet<String>,
    pub stat_bonuses: HashMap<String, f32>,
    pub active_quests: Vec<ActiveQuest>,
    pub spawned: bool,
    pub active_view: HashSet<ChunkCoord>, // Interest set
    pub input_dx: f32,
    pub input_dy: f32,
    pub input_attack: bool,
    pub input_interact: bool,
    pub input_aim: Option<Vec2>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settlement {
    pub id: Uuid,
    pub name: String,
    pub core_level: u32,
    pub core_integrity: f32,
    pub bounds_radius: f32,
    pub state: HashMap<String, serde_json::Value>,
}

pub struct World {
    pub seed: String,
    pub chunks: HashMap<ChunkCoord, Chunk>,
    pub settlements: HashMap<Uuid, Settlement>,
    pub players: HashMap<Uuid, PlayerState>,
    pub active_chunks: HashSet<ChunkCoord>,
    pub pending_notifications: Vec<(Uuid, String)>,
    pub config: Config,
}

impl World {
    pub fn new(config: Config) -> Self {
        Self {
            seed: config.world.seed.clone(),
            chunks: HashMap::new(),
            settlements: HashMap::new(),
            players: HashMap::new(),
            active_chunks: HashSet::new(),
            pending_notifications: Vec::new(),
            config,
        }
    }

    pub fn get_chunk(&self, coord: ChunkCoord) -> Option<&Chunk> {
        self.chunks.get(&coord)
    }

    pub fn get_chunk_mut(&mut self, coord: ChunkCoord) -> Option<&mut Chunk> {
        self.chunks.get_mut(&coord)
    }
}

pub mod gen;

pub fn init() {}