use config::SpawningConfig;
use world::{World, ChunkCoord};
use std::collections::HashMap;

pub struct SpawningSystem {
    respawn_timers: HashMap<(ChunkCoord, String), f64>,
}

impl SpawningSystem {
    pub fn new() -> Self {
        Self {
            respawn_timers: HashMap::new(),
        }
    }

    pub fn tick(&mut self, world: &mut World, config: &SpawningConfig, dt: f64) {
        // Update respawn timers
        let mut to_respawn: Vec<(ChunkCoord, String)> = Vec::new();
        
        for ((coord, entity_type), timer) in self.respawn_timers.iter_mut() {
            *timer -= dt;
            if *timer <= 0.0 {
                to_respawn.push((*coord, entity_type.clone()));
            }
        }
        
        // Remove expired timers
        for (coord, entity_type) in &to_respawn {
            self.respawn_timers.remove(&(*coord, entity_type.clone()));
        }
        
        // Spawn new entities in active chunks based on budgets
        for &coord in &world.active_chunks.clone() {
            if let Some(chunk) = world.get_chunk(coord) {
                let biome = &chunk.biome_id;
                
                if let Some(budget) = config.chunk_budgets.get(biome) {
                    // Check resource budgets
                    for (resource_type, target_count) in &budget.resources {
                        let current_count = chunk.resources.iter()
                            .filter(|e| e.subtype == *resource_type)
                            .count() as i32;
                        
                        if current_count < *target_count && 
                           !self.respawn_timers.contains_key(&(coord, resource_type.clone())) {
                            // Start respawn timer
                            let timer = *config.resource_respawn_seconds.get(resource_type)
                                .unwrap_or(&120) as f64;
                            self.respawn_timers.insert((coord, resource_type.clone()), timer);
                        }
                    }
                }
            }
        }
    }
}
