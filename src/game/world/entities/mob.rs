use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mob {
    pub m_type: String,
    pub level: u8,
    pub health: f32,
    pub max_health: f32,
    pub x: f32,
    pub y: f32,
    pub aggro: bool,
}

impl Mob {
    pub fn new(m_type: impl Into<String>, x: f32, y: f32) -> Self {
        Self {
            m_type: m_type.into(),
            level: 1,
            health: 40.0,
            max_health: 40.0,
            x,
            y,
            aggro: false,
        }
    }

    pub fn is_hostile(&self) -> bool {
        matches!(self.m_type.as_str(), "wolf" | "boar" | "bandit")
    }
}
