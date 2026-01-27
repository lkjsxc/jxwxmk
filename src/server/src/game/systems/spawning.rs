use rand::{Rng, SeedableRng};

use crate::config::Config;
use crate::game::chunk::{ChunkCoord, RespawnKind};
use crate::game::entities::{Mob, ResourceNode};
use crate::game::world::World;

pub fn tick_spawns(world: &mut World, config: &Config, delta_seconds: f32, tick: u64) {
    let chunk_size = config.world.chunk_size;
    let seed = world.seed;

    for chunk in world.chunks.values_mut() {
        let mut respawned = Vec::new();
        for (idx, entry) in chunk.cooldowns.iter_mut().enumerate() {
            entry.remaining -= delta_seconds;
            if entry.remaining <= 0.0 {
                respawned.push(idx);
            }
        }

        for idx in respawned.into_iter().rev() {
            let entry = chunk.cooldowns.remove(idx);
            match entry.kind {
                RespawnKind::Resource { r_type, level } => {
                    let resource = spawn_resource(seed, chunk.coord, tick, chunk_size, r_type, level);
                    chunk.resources.insert(resource.id.clone(), resource);
                }
                RespawnKind::Mob { m_type, level } => {
                    let mob = spawn_mob(seed, chunk.coord, tick, chunk_size, m_type, level);
                    chunk.mobs.insert(mob.id.clone(), mob);
                }
            }
        }
    }
}

pub fn spawn_resource(
    seed: u64,
    coord: ChunkCoord,
    tick: u64,
    chunk_size: f32,
    r_type: String,
    level: u32,
) -> ResourceNode {
    let mut rng = rng_for_chunk(seed, coord, tick);
    let x = coord.cx as f32 * chunk_size + rng.gen_range(0.0..chunk_size);
    let y = coord.cy as f32 * chunk_size + rng.gen_range(0.0..chunk_size);
    let id = format!("r_{}_{}_{}", coord.cx, coord.cy, rng.gen::<u32>());

    ResourceNode {
        id,
        r_type,
        level,
        amount: 30.0,
        x,
        y,
    }
}

pub fn spawn_mob(
    seed: u64,
    coord: ChunkCoord,
    tick: u64,
    chunk_size: f32,
    m_type: String,
    level: u32,
) -> Mob {
    let mut rng = rng_for_chunk(seed, coord, tick ^ 0x5a5a);
    let x = coord.cx as f32 * chunk_size + rng.gen_range(0.0..chunk_size);
    let y = coord.cy as f32 * chunk_size + rng.gen_range(0.0..chunk_size);
    let id = format!("m_{}_{}_{}", coord.cx, coord.cy, rng.gen::<u32>());

    Mob {
        id,
        m_type,
        level,
        health: 20.0 + level as f32 * 5.0,
        max_health: 20.0 + level as f32 * 5.0,
        x,
        y,
    }
}

fn rng_for_chunk(seed: u64, coord: ChunkCoord, tick: u64) -> rand::rngs::StdRng {
    let mix = seed
        ^ (coord.cx as u64).wrapping_mul(31_415_927)
        ^ (coord.cy as u64).wrapping_mul(1_005_973)
        ^ tick.wrapping_mul(265_443_5761);
    rand::rngs::StdRng::seed_from_u64(mix)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{
        AchievementsConfig, BalanceConfig, BiomesConfig, Config, CraftingConfig, EconomyConfig,
        QuestsConfig, ServerConfig, SettlementsConfig, SpawningConfig, SurvivalConfig, WorldConfig,
    };
    use crate::game::chunk::{Chunk, RespawnEntry};
    use crate::game::world::World;

    fn base_config() -> Config {
        Config::new(
            ServerConfig::default(),
            WorldConfig::default(),
            BalanceConfig::default(),
            SurvivalConfig::default(),
            CraftingConfig::default(),
            SpawningConfig::default(),
            BiomesConfig::default(),
            SettlementsConfig::default(),
            EconomyConfig::default(),
            QuestsConfig::default(),
            AchievementsConfig::default(),
        )
    }

    #[test]
    fn respawns_resource_after_cooldown() {
        let config = base_config();
        let mut world = World::new(0);
        let coord = ChunkCoord::new(0, 0);
        let mut chunk = Chunk::new(coord, "forest".to_string());
        chunk.cooldowns.push(RespawnEntry {
            kind: RespawnKind::Resource {
                r_type: "tree".to_string(),
                level: 1,
            },
            remaining: 0.0,
        });
        world.chunks.insert(coord, chunk);

        tick_spawns(&mut world, &config, 1.0, 1);
        let chunk = world.chunks.get(&coord).unwrap();
        assert!(!chunk.resources.is_empty());
    }
}
