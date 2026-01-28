use world::{World, ChunkCoord, Vec2};
use std::time::Duration;
use rand::Rng;

pub fn tick(world: &mut World, dt: Duration) {
    let dt_secs = dt.as_secs_f32();
    let chunk_size = world.config.world.chunk_size_wu as f32;
    let mut rng = rand::thread_rng();

    let active_chunks: Vec<ChunkCoord> = world.active_chunks.iter().cloned().collect();

    for coord in active_chunks {
        if let Some(chunk) = world.get_chunk_mut(coord) {
            for mob in chunk.mobs.values_mut() {
                // Basic wandering AI
                if rng.gen_bool(0.01) { // 1% chance per tick to change direction
                    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                    mob.target_pos = Some(Vec2 {
                        x: mob.pos.x + angle.cos() * 5.0,
                        y: mob.pos.y + angle.sin() * 5.0,
                    });
                }

                if let Some(target) = mob.target_pos {
                    let dx = target.x - mob.pos.x;
                    let dy = target.y - mob.pos.y;
                    let dist = (dx*dx + dy*dy).sqrt();

                    if dist < 0.1 {
                        mob.target_pos = None;
                    } else {
                        mob.pos.x += (dx / dist) * 2.0 * dt_secs;
                        mob.pos.y += (dy / dist) * 2.0 * dt_secs;
                    }
                }
            }
        }
    }
}
