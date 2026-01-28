use world::{World, ChunkCoord};
use std::time::Duration;

pub fn tick(world: &mut World, _dt: Duration) {
    let barrier_cfg = world.config.settlements.barrier.clone();
    
    let mut safe_zones = Vec::new();
    for chunk in world.chunks.values() {
        for struct_entity in chunk.structures.values() {
            if struct_entity.subtype == "barrier_core" {
                let level = struct_entity.hp / 100.0;
                let range = barrier_cfg.base_range_wu + (level - 1.0) * barrier_cfg.level_multiplier_wu;
                let world_pos = world_to_global(chunk.coord, struct_entity.pos, world.config.world.chunk_size_wu as f32);
                safe_zones.push((world_pos, range));
            }
        }
    }

    for chunk in world.chunks.values_mut() {
        chunk.mobs.retain(|_id, mob| {
            let mob_world_pos = world_to_global(chunk.coord, mob.pos, world.config.world.chunk_size_wu as f32);
            for (zone_pos, radius) in &safe_zones {
                let dx = mob_world_pos.x - zone_pos.x;
                let dy = mob_world_pos.y - zone_pos.y;
                if dx*dx + dy*dy < radius*radius {
                    return false;
                }
            }
            true
        });
    }
}

fn world_to_global(coord: ChunkCoord, local_pos: world::Vec2, chunk_size: f32) -> world::Vec2 {
    world::Vec2 {
        x: coord.x as f32 * chunk_size + local_pos.x,
        y: coord.y as f32 * chunk_size + local_pos.y,
    }
}