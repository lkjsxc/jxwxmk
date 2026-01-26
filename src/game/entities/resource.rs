use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResourceType {
    Tree,
    Rock,
    Food,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub id: Uuid,
    pub r_type: ResourceType,
    pub x: f64,
    pub y: f64,
    pub amount: i32, // Remaining health/resources
}

impl Resource {
    pub fn new(r_type: ResourceType, x: f64, y: f64) -> Self {
        let amount = match r_type {
            ResourceType::Tree => 5,
            ResourceType::Rock => 10,
            ResourceType::Food => 1,
        };
        Self {
            id: Uuid::new_v4(),
            r_type,
            x,
            y,
            amount,
        }
    }
}
