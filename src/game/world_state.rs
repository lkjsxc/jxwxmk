use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemType {
    Wood,
    Stone,
    Gold,
    Diamond,
    Berry,
    Meat,
    CookedMeat,
    WoodPickaxe,
    StonePickaxe,
    WoodWall,
    Door,
    Torch,
    Workbench,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    pub kind: ItemType,
    pub amount: u32,
    pub max_stack: u32,
    pub level: u32,
    pub xp: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Inventory {
    pub slots: Vec<Option<Item>>,
}

impl Inventory {
    pub fn new(size: usize) -> Self {
        Self {
            slots: vec![None; size],
        }
    }
}

use crate::game::quests::QuestState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: Uuid,
    pub token: Uuid,
    pub username: String,
    pub x: f64,
    pub y: f64,
    pub health: f64,
    pub hunger: f64,
    pub cold: f64,
    pub inventory: Inventory,
    pub active_slot: usize,
    pub stats: PlayerStats,
    pub achievements: HashSet<String>,
    pub quests: HashMap<String, QuestState>,
    pub stat_bonuses: HashMap<String, f64>,
    pub spawned: bool,
    #[serde(skip)]
    pub last_attack_at: f64,
    #[serde(skip)]
    pub last_interact_at: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PlayerStats {
    pub steps: u64,
    pub kills: u64,
    pub crafts: u64,
    pub gathers: u64,
    pub structures: u64,
    pub damage_taken: f64,
    pub deaths: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Tree,
    Rock,
    Food,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resource {
    pub id: Uuid,
    pub r_type: ResourceType,
    pub x: f64,
    pub y: f64,
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum MobType {
    Rabbit,
    Wolf,
    Bear,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mob {
    pub id: Uuid,
    pub m_type: MobType,
    pub x: f64,
    pub y: f64,
    pub health: f64,
    pub level: u32,
    pub target_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum StructureType {
    Wall,
    Door,
    Torch,
    Workbench,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Structure {
    pub id: Uuid,
    pub s_type: StructureType,
    pub x: f64,
    pub y: f64,
    pub health: f64,
    pub owner_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum NpcType {
    Elder,
    Merchant,
    Guard,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Npc {
    pub id: Uuid,
    pub n_type: NpcType,
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub health: f64,
    pub dialogue_index: u32,
    pub trade_inventory: Option<Vec<Item>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BarrierCore {
    pub id: Uuid,
    pub x: f64,
    pub y: f64,
    pub level: u32,
    pub base_range: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct Effect {

    pub id: Uuid,

    pub x: f64,

    pub y: f64,

    pub text: String,

    pub color: String,

    pub ttl: u32, // ticks to live

}



#[derive(Debug, Serialize, Default, Clone)]

pub struct World {

    pub width: f64,

    pub height: f64,

    pub players: HashMap<Uuid, Player>,

    pub resources: HashMap<Uuid, Resource>,

    pub mobs: HashMap<Uuid, Mob>,

    pub structures: HashMap<Uuid, Structure>,

    pub npcs: HashMap<Uuid, Npc>,

    pub barrier_cores: HashMap<Uuid, BarrierCore>,

    pub effects: HashMap<Uuid, Effect>,

}
