use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarrierCore {
    pub id: Uuid,
    pub x: f64,
    pub y: f64,
    pub level: u32,
    pub base_range: f64,
}

impl BarrierCore {
    pub fn new(x: f64, y: f64, level: u32, base_range: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            x,
            y,
            level,
            base_range,
        }
    }

    pub fn range(&self, level_multiplier: f64) -> f64 {
        self.base_range + (self.level as f64 - 1.0) * level_multiplier
    }
}
