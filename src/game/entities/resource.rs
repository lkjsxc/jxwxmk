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

use crate::game::config::ResourceConfig;

impl Resource {
    pub fn new(r_type: ResourceType, x: f64, y: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            r_type,
            x,
            y,
            amount: 1, // Default
        }
    }

    pub fn new_with_config(r_type: ResourceType, x: f64, y: f64, cfg: &ResourceConfig) -> Self {
        let amount = match r_type {
            ResourceType::Tree => cfg.tree_amount,
            ResourceType::Rock => cfg.rock_amount,
            ResourceType::Food => cfg.food_amount,
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
