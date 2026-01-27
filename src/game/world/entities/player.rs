use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use super::inventory::Inventory;
use super::stats::Stats;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerState {
    pub id: Uuid,
    pub token: Uuid,
    pub username: String,
    pub x: f32,
    pub y: f32,
    pub chunk_x: i32,
    pub chunk_y: i32,
    pub health: f32,
    pub hunger: f32,
    pub temperature: f32,
    pub thirst: Option<f32>,
    pub inventory: Inventory,
    pub stats: Stats,
    pub level: u32,
    pub xp: u64,
    pub achievements: HashSet<String>,
    pub quests: Vec<PlayerQuest>,
    pub reputation: HashMap<String, i32>,
    pub spawned: bool,
    pub bound_settlement: Option<Uuid>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerQuest {
    pub id: String,
    pub name: String,
    pub state: String,
    pub objectives: Vec<PlayerQuestObjective>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerQuestObjective {
    pub kind: String,
    pub target: String,
    pub count: u32,
    pub current: u32,
}

impl PlayerState {
    pub fn new(id: Uuid, token: Uuid, inventory_slots: usize) -> Self {
        Self {
            id,
            token,
            username: "Wanderer".to_string(),
            x: 0.0,
            y: 0.0,
            chunk_x: 0,
            chunk_y: 0,
            health: 100.0,
            hunger: 100.0,
            temperature: 50.0,
            thirst: None,
            inventory: Inventory::new(inventory_slots),
            stats: Stats::default(),
            level: 1,
            xp: 0,
            achievements: HashSet::new(),
            quests: Vec::new(),
            reputation: HashMap::new(),
            spawned: false,
            bound_settlement: None,
        }
    }
}
