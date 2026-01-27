use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Npc {
    pub role: String,
    pub faction: String,
    pub name: String,
    pub x: f32,
    pub y: f32,
}

impl Npc {
    pub fn new(role: impl Into<String>, name: impl Into<String>, x: f32, y: f32) -> Self {
        Self {
            role: role.into(),
            faction: "neutral".to_string(),
            name: name.into(),
            x,
            y,
        }
    }
}
