use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BarrierCore {
    pub level: u8,
    pub base_range: f32,
    pub faction: String,
    pub integrity: f32,
}

impl BarrierCore {
    pub fn range(&self, level_multiplier: f32) -> f32 {
        self.base_range + (self.level.saturating_sub(1) as f32) * level_multiplier
    }
}
