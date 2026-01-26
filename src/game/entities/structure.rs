use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StructureType {
    Wall,
    Door,
    Torch,
    Workbench,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Structure {
    pub id: Uuid,
    pub s_type: StructureType,
    pub x: f64,
    pub y: f64,
    pub health: f64,
    pub owner_id: Uuid,
}

use crate::game::config::StructureConfig;

impl Structure {
    pub fn new(s_type: StructureType, x: f64, y: f64, owner_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            s_type,
            x,
            y,
            health: 10.0, // Default
            owner_id,
        }
    }

    pub fn new_with_config(s_type: StructureType, x: f64, y: f64, owner_id: Uuid, cfg: &StructureConfig) -> Self {
        let health = match s_type {
            StructureType::Wall => cfg.wall_health,
            StructureType::Door => cfg.door_health,
            StructureType::Workbench => cfg.workbench_health,
            StructureType::Torch => cfg.torch_health,
        };
        Self {
            id: Uuid::new_v4(),
            s_type,
            x,
            y,
            health,
            owner_id,
        }
    }
}
