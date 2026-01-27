use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Structure {
    pub id: String,
    pub s_type: String,
    pub tier: u32,
    pub health: f32,
    pub max_health: f32,
    pub owner_id: Option<String>,
    pub x: f32,
    pub y: f32,
}
