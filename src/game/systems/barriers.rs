use crate::config::Config;
use crate::game::world::World;

pub fn enforce(world: &mut World, _config: &Config) {
    let settlements = world.settlements.values().cloned().collect::<Vec<_>>();
    for settlement in settlements {
        for chunk in world.chunks.values_mut() {
            let mut to_remove = Vec::new();
            for (id, mob) in &chunk.mobs {
                if mob.is_hostile() {
                    let dx = mob.x - settlement.center_x;
                    let dy = mob.y - settlement.center_y;
                    if (dx * dx + dy * dy).sqrt() <= settlement.safe_radius {
                        to_remove.push(*id);
                    }
                }
            }
            for id in to_remove {
                chunk.mobs.remove(&id);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::game::world::{coords::ChunkCoord, entities::Mob, World};
    use uuid::Uuid;

    #[test]
    fn safe_zone_removes_hostiles() {
        let config = Config::default();
        let mut world = World::new(config.world.seed);
        let settlement = world.spawn_settlement(&config);
        let coord = ChunkCoord::new(0, 0);
        let chunk = world.ensure_chunk(coord, &config);
        let mob_id = Uuid::new_v4();
        chunk.mobs.insert(mob_id, Mob::new("wolf", settlement.center_x, settlement.center_y));
        enforce(&mut world, &config);
        let chunk = world.chunks.get(&coord).unwrap();
        assert!(chunk.mobs.is_empty());
    }
}
