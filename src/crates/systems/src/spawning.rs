use world::{World, ChunkCoord};
use std::time::{Duration, Instant};

pub fn tick(world: &mut World, _dt: Duration) {
    let now = Instant::now();
    let mut chunks_to_process: Vec<ChunkCoord> = world.active_chunks.iter().cloned().collect();

    for coord in chunks_to_process {
        if let Some(chunk) = world.get_chunk_mut(coord) {
            // Process respawn queue
            let mut ready = Vec::new();
            chunk.respawn_queue.retain(|(time, ent)| {
                if now >= *time {
                    ready.push(ent.clone());
                    false
                } else {
                    true
                }
            });

            for mut ent in ready {
                ent.hp = ent.max_hp;
                match ent.kind {
                    world::EntityKind::Resource => { chunk.resources.insert(ent.id, ent); }
                    world::EntityKind::Mob => { chunk.mobs.insert(ent.id, ent); }
                    _ => {}
                }
                chunk.dirty = true;
            }
        }
    }
}
