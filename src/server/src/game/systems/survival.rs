use crate::config::Config;
use crate::util::clamp_f32;

use crate::game::world::World;

pub fn tick_survival(world: &mut World, config: &Config, delta_seconds: f32) {
    for player in world.players.values_mut() {
        if !player.spawned {
            continue;
        }

        let biome_mod = world
            .chunks
            .get(&crate::game::chunk::ChunkCoord::new(player.chunk.0, player.chunk.1))
            .and_then(|chunk| {
                config
                    .biomes
                    .biomes
                    .iter()
                    .find(|biome| biome.id == chunk.biome_id)
            });

        let hunger_mod = biome_mod.map(|b| b.hunger_modifier).unwrap_or(1.0);
        let temp_mod = biome_mod.map(|b| b.temperature_modifier).unwrap_or(0.0);

        player.hunger -= config.survival.hunger_decay * hunger_mod * delta_seconds;
        if player.hunger <= 0.0 {
            player.health -= config.survival.starve_damage * delta_seconds;
        } else if player.hunger >= config.survival.heal_threshold {
            player.health += config.survival.heal_rate * delta_seconds;
        }

        let target_temp = config.survival.neutral_temp + temp_mod;
        let temp_delta = (target_temp - player.temperature) * 0.1 * delta_seconds;
        player.temperature += temp_delta;
        if player.temperature <= 0.0 {
            player.health -= config.survival.freeze_damage * delta_seconds;
        }

        if config.survival.thirst_enabled {
            player.thirst -= config.survival.thirst_decay * delta_seconds;
            if player.thirst <= 0.0 {
                player.health -= config.survival.thirst_damage * delta_seconds;
            }
        }

        player.health = clamp_f32(player.health, 0.0, config.balance.player.max_health);
        player.hunger = clamp_f32(player.hunger, 0.0, 100.0);
        player.temperature = clamp_f32(player.temperature, 0.0, 100.0);
        player.thirst = clamp_f32(player.thirst, 0.0, 100.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{
        AchievementsConfig, BalanceConfig, BiomesConfig, Config, CraftingConfig, EconomyConfig,
        QuestsConfig, ServerConfig, SettlementsConfig, SpawningConfig, SurvivalConfig, WorldConfig,
    };
    use crate::game::entities::PlayerState;
    use crate::game::world::World;
    use uuid::Uuid;

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
    fn hunger_decay_applies_damage_at_zero() {
        let config = base_config();
        let mut world = World::new(0);
        let mut player = PlayerState::new(Uuid::new_v4(), Uuid::new_v4(), 5, 100.0);
        player.spawned = true;
        player.hunger = 0.0;
        player.health = 50.0;
        world.upsert_player(player);

        tick_survival(&mut world, &config, 1.0);
        let player = world.players.values().next().unwrap();
        assert!(player.health < 50.0);
    }
}
