use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldConfig {
    pub chunk_size: f32,
    pub view_radius: i32,
    pub simulation_radius: i32,
    pub seed: u64,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            chunk_size: 64.0,
            view_radius: 3,
            simulation_radius: 4,
            seed: 1337,
        }
    }
}
