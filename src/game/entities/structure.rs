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

impl Structure {
    pub fn new(s_type: StructureType, x: f64, y: f64, owner_id: Uuid) -> Self {
        let health = match s_type {
            StructureType::Wall => 200.0,
            StructureType::Door => 100.0,
            StructureType::Workbench => 50.0,
            StructureType::Torch => 10.0,
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
