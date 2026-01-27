use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct SpawningConfig {
    pub max_resources_per_chunk: usize,
    pub max_mobs_per_chunk: usize,
    pub resource_respawn_seconds: f32,
    pub mob_respawn_seconds: f32,
}

impl Default for SpawningConfig {
    fn default() -> Self {
        Self {
            max_resources_per_chunk: 12,
            max_mobs_per_chunk: 6,
            resource_respawn_seconds: 30.0,
            mob_respawn_seconds: 45.0,
        }
    }
}
