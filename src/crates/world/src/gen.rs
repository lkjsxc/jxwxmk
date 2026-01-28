use crate::{Chunk, ChunkCoord, Entity, EntityKind, Vec2};
use config::Config;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use uuid::Uuid;

pub fn generate_chunk(coord: ChunkCoord, world_seed: &str, config: &Config) -> Chunk {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    std::hash::Hash::hash(&world_seed, &mut hasher);
    std::hash::Hash::hash(&coord.x, &mut hasher);
    std::hash::Hash::hash(&coord.y, &mut hasher);
    let chunk_seed = std::hash::Hasher::finish(&hasher);
    
    let mut rng = ChaCha8Rng::seed_from_u64(chunk_seed);

    let biome_id = if config.biomes.biomes.is_empty() {
        "forest".to_string()
    } else {
        let idx = rng.gen_range(0..config.biomes.biomes.len());
        config.biomes.biomes[idx].id.clone()
    };

    let mut chunk = Chunk::new(coord, biome_id.clone());

    // --- Baseline Village (coord [0,0]) ---
    if coord.x == 0 && coord.y == 0 {
        spawn_village(&mut chunk);
    }

    if let Some(budget) = config.spawning.chunk_budgets.get(&biome_id) {
        for (subtype, &count) in &budget.resources {
            for _ in 0..count {
                let x = rng.gen_range(0.0..config.world.chunk_size_wu as f32);
                let y = rng.gen_range(0.0..config.world.chunk_size_wu as f32);
                let id = Uuid::new_v4();
                chunk.resources.insert(id, Entity {
                    id, kind: EntityKind::Resource, subtype: subtype.clone(),
                    pos: Vec2 { x, y }, hp: 30.0, max_hp: 30.0, name: None, owner_id: None,
                    target_pos: None,
                });
            }
        }
        for (subtype, &count) in &budget.mobs {
            for _ in 0..count {
                let x = rng.gen_range(0.0..config.world.chunk_size_wu as f32);
                let y = rng.gen_range(0.0..config.world.chunk_size_wu as f32);
                let id = Uuid::new_v4();
                chunk.mobs.insert(id, Entity {
                    id, kind: EntityKind::Mob, subtype: subtype.clone(),
                    pos: Vec2 { x, y }, hp: 50.0, max_hp: 50.0, name: None, owner_id: None,
                    target_pos: None,
                });
            }
        }
    }

    chunk
}

fn spawn_village(chunk: &mut Chunk) {
    // Barrier Core (Structure)
    let core_id = Uuid::new_v4();
    chunk.structures.insert(core_id, Entity {
        id: core_id,
        kind: EntityKind::Structure,
        subtype: "barrier_core".into(),
        pos: Vec2 { x: 64.0, y: 64.0 }, // Center
        hp: 1000.0, max_hp: 1000.0,
        name: Some("Origin Village Core".into()),
        owner_id: None,
        target_pos: None,
    });

    // Trader NPC
    let trader_id = Uuid::new_v4();
    chunk.npcs.insert(trader_id, Entity {
        id: trader_id,
        kind: EntityKind::Npc,
        subtype: "trader".into(),
        pos: Vec2 { x: 70.0, y: 64.0 },
        hp: 100.0, max_hp: 100.0,
        name: Some("Trader Lina".into()),
        owner_id: None,
        target_pos: None,
    });

    // Quest Giver NPC
    let quest_id = Uuid::new_v4();
    chunk.npcs.insert(quest_id, Entity {
        id: quest_id,
        kind: EntityKind::Npc,
        subtype: "quest_giver".into(),
        pos: Vec2 { x: 64.0, y: 70.0 },
        hp: 100.0, max_hp: 100.0,
        name: Some("Elder Silas".into()),
        owner_id: None,
        target_pos: None,
    });
}
