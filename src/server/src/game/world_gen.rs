use rand::{Rng, SeedableRng};

use crate::config::Config;
use crate::game::chunk::{Chunk, ChunkCoord};
use crate::game::entities::{Mob, Npc, ResourceNode};

pub fn generate_chunk(seed: u64, coord: ChunkCoord, config: &Config, tick: u64) -> Chunk {
    let mut rng = chunk_rng(seed, coord, tick);
    let biome = pick_biome(&config.biomes, &mut rng).unwrap_or_else(|| "forest".to_string());
    let mut chunk = Chunk::new(coord, biome);

    for index in 0..config.spawning.max_resources_per_chunk {
        let r_type = match index % 3 {
            0 => "tree",
            1 => "rock",
            _ => "food",
        };
        let resource = ResourceNode {
            id: format!("r_{}_{}_{}", coord.cx, coord.cy, index),
            r_type: r_type.to_string(),
            level: 1,
            amount: 30.0,
            x: coord.cx as f32 * config.world.chunk_size + rng.gen_range(0.0..config.world.chunk_size),
            y: coord.cy as f32 * config.world.chunk_size + rng.gen_range(0.0..config.world.chunk_size),
        };
        chunk.resources.insert(resource.id.clone(), resource);
    }

    for index in 0..config.spawning.max_mobs_per_chunk {
        let mob = Mob {
            id: format!("m_{}_{}_{}", coord.cx, coord.cy, index),
            m_type: "wolf".to_string(),
            level: 1,
            health: 20.0,
            max_health: 20.0,
            x: coord.cx as f32 * config.world.chunk_size + rng.gen_range(0.0..config.world.chunk_size),
            y: coord.cy as f32 * config.world.chunk_size + rng.gen_range(0.0..config.world.chunk_size),
        };
        chunk.mobs.insert(mob.id.clone(), mob);
    }

    if coord.cx == 0 && coord.cy == 0 {
        let npc = Npc {
            id: "npc_trader".to_string(),
            role: "trader".to_string(),
            faction: "neutral".to_string(),
            name: "Trader Lina".to_string(),
            x: 5.0,
            y: 5.0,
        };
        chunk.npcs.insert(npc.id.clone(), npc);
    }

    chunk
}

fn pick_biome(biomes: &crate::config::BiomesConfig, rng: &mut impl Rng) -> Option<String> {
    if biomes.biomes.is_empty() {
        return None;
    }
    let idx = rng.gen_range(0..biomes.biomes.len());
    Some(biomes.biomes[idx].id.clone())
}

fn chunk_rng(seed: u64, coord: ChunkCoord, tick: u64) -> rand::rngs::StdRng {
    let mix = seed
        ^ (coord.cx as u64).wrapping_mul(13_721)
        ^ (coord.cy as u64).wrapping_mul(19_133)
        ^ tick.wrapping_mul(97_531);
    rand::rngs::StdRng::seed_from_u64(mix)
}
