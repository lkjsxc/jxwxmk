use crate::{Chunk, Resource};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::collections::HashMap;

pub struct ResourceSpawner {
    seed: u64,
}

#[derive(Debug, Clone)]
pub struct SpawnBudget {
    pub max_resources: usize,
    pub respawn_interval_secs: f32,
}

impl ResourceSpawner {
    pub fn new(seed: u64) -> Self {
        ResourceSpawner { seed }
    }

    pub fn get_budget_for_biome(biome_id: &str) -> SpawnBudget {
        match biome_id {
            "forest" => SpawnBudget {
                max_resources: 15,
                respawn_interval_secs: 300.0,
            },
            "plains" => SpawnBudget {
                max_resources: 8,
                respawn_interval_secs: 240.0,
            },
            "desert" => SpawnBudget {
                max_resources: 4,
                respawn_interval_secs: 600.0,
            },
            "tundra" => SpawnBudget {
                max_resources: 6,
                respawn_interval_secs: 480.0,
            },
            "mountains" => SpawnBudget {
                max_resources: 12,
                respawn_interval_secs: 360.0,
            },
            _ => SpawnBudget {
                max_resources: 10,
                respawn_interval_secs: 300.0,
            },
        }
    }

    pub fn update(
        &self,
        chunk: &mut Chunk,
        dt: f32,
    ) {
        let budget = Self::get_budget_for_biome(&chunk.biome_id);
        
        // Update cooldowns
        for cooldown in chunk.cooldowns.values_mut() {
            *cooldown -= dt;
        }
        chunk.cooldowns.retain(|_, v| *v > 0.0);
        
        // Check if we need to spawn new resources
        let current_count = chunk.resources.len();
        if current_count < budget.max_resources {
            let to_spawn = budget.max_resources - current_count;
            self.spawn_resources(chunk, to_spawn, &budget);
        }
    }

    fn spawn_resources(
        &self,
        chunk: &mut Chunk,
        count: usize,
        budget: &SpawnBudget,
    ) {
        use rand::Rng;
        
        let mut rng = StdRng::seed_from_u64(
            self.seed.wrapping_add(
                (chunk.coord.0 as u64).wrapping_mul(99991)
                    .wrapping_add((chunk.coord.1 as u64).wrapping_mul(99989))
            )
        );

        for i in 0..count {
            let resource_type = self.select_resource_type(&chunk.biome_id, &mut rng);
            
            let x = rng.gen_range(0.0..128.0);
            let y = rng.gen_range(0.0..128.0);
            
            let resource_id = format!("resource_{}_{}_{}", chunk.coord.0, chunk.coord.1, i);
            
            let resource = Resource {
                id: resource_id.clone(),
                subtype: resource_type.to_string(),
                x,
                y,
                hp: 30.0,
                max_hp: 30.0,
                level: 1,
            };
            
            chunk.resources.insert(resource_id, resource);
            chunk.mark_dirty();
        }
    }

    fn select_resource_type<R: Rng>(
        &self,
        biome_id: &str,
        rng: &mut R,
    ) -> &'static str {
        match biome_id {
            "forest" => {
                if rng.gen_bool(0.7) {
                    "tree"
                } else {
                    "rock"
                }
            }
            "plains" => {
                if rng.gen_bool(0.6) {
                    "bush"
                } else {
                    "rock"
                }
            }
            "desert" => {
                if rng.gen_bool(0.5) {
                    "cactus"
                } else {
                    "sandstone"
                }
            }
            "tundra" => {
                if rng.gen_bool(0.6) {
                    "snow_rock"
                } else {
                    "ice"
                }
            }
            "mountains" => {
                if rng.gen_bool(0.7) {
                    "rock"
                } else {
                    "ore"
                }
            }
            _ => "rock",
        }
    }

    pub fn on_resource_depleted(
        &self,
        chunk: &mut Chunk,
        resource_id: &str,
    ) {
        chunk.resources.remove(resource_id);
        
        let budget = Self::get_budget_for_biome(&chunk.biome_id);
        chunk.cooldowns.insert(
            resource_id.to_string(),
            budget.respawn_interval_secs,
        );
        
        chunk.mark_dirty();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Chunk;

    #[test]
    fn test_spawn_budget_varies_by_biome() {
        let forest_budget = ResourceSpawner::get_budget_for_biome("forest");
        let desert_budget = ResourceSpawner::get_budget_for_biome("desert");
        
        assert!(forest_budget.max_resources > desert_budget.max_resources);
    }

    #[test]
    fn test_resource_spawned_when_below_budget() {
        let spawner = ResourceSpawner::new(12345);
        let mut chunk = Chunk::new((0, 0));
        chunk.biome_id = "forest".to_string();
        
        let initial_count = chunk.resources.len();
        spawner.update(&mut chunk, 1.0);
        
        assert!(chunk.resources.len() > initial_count);
    }

    #[test]
    fn test_cooldown_added_on_depletion() {
        let spawner = ResourceSpawner::new(12345);
        let mut chunk = Chunk::new((0, 0));
        chunk.biome_id = "forest".to_string();
        
        // Add a resource first
        chunk.resources.insert(
            "test_resource".to_string(),
            Resource {
                id: "test_resource".to_string(),
                subtype: "tree".to_string(),
                x: 10.0,
                y: 10.0,
                hp: 30.0,
                max_hp: 30.0,
                level: 1,
            },
        );
        
        spawner.on_resource_depleted(&mut chunk, "test_resource");
        
        assert!(chunk.cooldowns.contains_key("test_resource"));
        assert!(!chunk.resources.contains_key("test_resource"));
    }
}
