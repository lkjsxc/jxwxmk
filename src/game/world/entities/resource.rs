use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceNode {
    pub r_type: String,
    pub level: u8,
    pub amount: u32,
    pub respawn_at: f32,
    pub x: f32,
    pub y: f32,
    pub health: f32,
    pub max_health: f32,
}

impl ResourceNode {
    pub fn new(r_type: impl Into<String>, x: f32, y: f32) -> Self {
        Self {
            r_type: r_type.into(),
            level: 1,
            amount: 3,
            respawn_at: 0.0,
            x,
            y,
            health: 30.0,
            max_health: 30.0,
        }
    }
}
