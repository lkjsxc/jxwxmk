use crate::game::world_state::{World, MobType};
use crate::config::AppConfig;

pub fn tick_barriers(world: &mut World) {
    let config = AppConfig::get();
    
    // Collect mobs to remove
    let mut to_remove = Vec::new();

    for mob in world.mobs.values() {
        // Only hostile mobs
        if matches!(mob.m_type, MobType::Rabbit) {
            continue;
        }

        for core in world.barrier_cores.values() {
            let range = core.base_range + ((core.level as f64 - 1.0) * config.barriers.level_multiplier);
            let dist = ((mob.x - core.x).powi(2) + (mob.y - core.y).powi(2)).sqrt();
            
            if dist <= range {
                to_remove.push(mob.id);
                break; // Mob is dead, stop checking other cores
            }
        }
    }

    for id in to_remove {
        world.mobs.remove(&id);
    }
}
