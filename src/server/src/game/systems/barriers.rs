use crate::game::world::World;

pub fn enforce_barriers(world: &mut World) {
    if world.settlements.is_empty() {
        return;
    }

    let cores: Vec<(f32, f32, f32)> = world
        .settlements
        .values()
        .map(|settlement| {
            let range = settlement.core.range();
            (settlement.core.x, settlement.core.y, range * range)
        })
        .collect();

    for chunk in world.chunks.values_mut() {
        let mobs_to_remove: Vec<String> = chunk
            .mobs
            .iter()
            .filter(|(_, mob)| {
                cores.iter().any(|(x, y, range_sq)| {
                    let dx = mob.x - x;
                    let dy = mob.y - y;
                    dx * dx + dy * dy <= *range_sq
                })
            })
            .map(|(id, _)| id.clone())
            .collect();

        for id in mobs_to_remove {
            chunk.mobs.remove(&id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{
        AchievementsConfig, BalanceConfig, BiomesConfig, Config, CraftingConfig, EconomyConfig,
        QuestsConfig, ServerConfig, SettlementsConfig, SpawningConfig, SurvivalConfig, WorldConfig,
    };
    use crate::game::chunk::{Chunk, ChunkCoord};
    use crate::game::entities::{BarrierCore, Mob};
    use crate::game::world::{Settlement, World};

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
    fn removes_mobs_inside_safe_zone() {
        let _config = base_config();
        let mut world = World::new(0);
        let core = BarrierCore {
            id: "core".to_string(),
            level: 2,
            base_range: 10.0,
            level_multiplier: 0.0,
            faction: "neutral".to_string(),
            integrity: 100.0,
            x: 0.0,
            y: 0.0,
        };
        world.settlements.insert(
            "settlement".to_string(),
            Settlement {
                id: "settlement".to_string(),
                name: "Test".to_string(),
                core,
                spawn_x: 0.0,
                spawn_y: 0.0,
            },
        );
        let coord = ChunkCoord::new(0, 0);
        let mut chunk = Chunk::new(coord, "forest".to_string());
        chunk.mobs.insert(
            "mob".to_string(),
            Mob {
                id: "mob".to_string(),
                m_type: "wolf".to_string(),
                level: 1,
                health: 10.0,
                max_health: 10.0,
                x: 0.0,
                y: 0.0,
            },
        );
        world.chunks.insert(coord, chunk);

        enforce_barriers(&mut world);
        let chunk = world.chunks.get(&coord).unwrap();
        assert!(chunk.mobs.is_empty());
    }
}
