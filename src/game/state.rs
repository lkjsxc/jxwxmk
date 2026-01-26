use std::collections::HashMap;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::game::entities::{player::Player, resource::Resource, mob::Mob, structure::Structure, npc::Npc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub width: f64,
    pub height: f64,
    pub players: HashMap<Uuid, Player>,
    pub resources: HashMap<Uuid, Resource>,
    pub mobs: HashMap<Uuid, Mob>,
    pub structures: HashMap<Uuid, Structure>,
    pub npcs: HashMap<Uuid, Npc>,
}

impl World {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width,
            height,
            players: HashMap::new(),
            resources: HashMap::new(),
            mobs: HashMap::new(),
            structures: HashMap::new(),
            npcs: HashMap::new(),
        }
    }
}
