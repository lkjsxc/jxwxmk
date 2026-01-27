use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceNode {
    pub id: String,
    pub r_type: String,
    pub level: u32,
    pub amount: f32,
    pub x: f32,
    pub y: f32,
}
