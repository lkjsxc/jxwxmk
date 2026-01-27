use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Npc {
    pub id: String,
    pub role: String,
    pub faction: String,
    pub name: String,
    pub x: f32,
    pub y: f32,
}
