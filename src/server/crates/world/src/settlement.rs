use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Settlement {
    pub id: Uuid,
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub core_level: i32,
    pub core_integrity: f64,
    pub tier: String,
}

impl Settlement {
    pub fn new(id: Uuid, name: String, x: f64, y: f64) -> Self {
        Self {
            id,
            name,
            x,
            y,
            core_level: 1,
            core_integrity: 100.0,
            tier: "outpost".to_string(),
        }
    }

    pub fn barrier_range(&self, base_range: f64, level_mult: f64) -> f64 {
        base_range + (self.core_level - 1) as f64 * level_mult
    }

    pub fn is_in_safe_zone(&self, x: f64, y: f64, base_range: f64, level_mult: f64) -> bool {
        let dx = x - self.x;
        let dy = y - self.y;
        let dist_sq = dx * dx + dy * dy;
        let range = self.barrier_range(base_range, level_mult);
        dist_sq <= range * range
    }
}

pub fn generate_initial_settlement() -> Settlement {
    Settlement::new(
        Uuid::new_v4(),
        "Starting Village".to_string(),
        0.0,
        0.0,
    )
}
