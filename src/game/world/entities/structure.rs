use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Structure {
    pub s_type: String,
    pub tier: u8,
    pub health: f32,
    pub max_health: f32,
    pub owner_id: Option<Uuid>,
    pub x: f32,
    pub y: f32,
}

impl Structure {
    pub fn new(s_type: impl Into<String>, x: f32, y: f32) -> Self {
        Self {
            s_type: s_type.into(),
            tier: 1,
            health: 100.0,
            max_health: 100.0,
            owner_id: None,
            x,
            y,
        }
    }
}
