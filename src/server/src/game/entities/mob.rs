use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mob {
    pub id: String,
    pub m_type: String,
    pub level: u32,
    pub health: f32,
    pub max_health: f32,
    pub x: f32,
    pub y: f32,
}
