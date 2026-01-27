use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarrierCore {
    pub id: String,
    pub level: u32,
    pub base_range: f32,
    pub level_multiplier: f32,
    pub faction: String,
    pub integrity: f32,
    pub x: f32,
    pub y: f32,
}

impl BarrierCore {
    pub fn range(&self) -> f32 {
        self.base_range + (self.level.saturating_sub(1) as f32) * self.level_multiplier
    }
}
