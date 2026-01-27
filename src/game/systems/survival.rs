use crate::config::Config;
use crate::game::world::entities::PlayerState;
use crate::game::world::World;

pub fn tick(world: &mut World, config: &Config, tick_rate: f32) {
    for player in world.players.values_mut() {
        if !player.spawned {
            continue;
        }
        player.hunger -= config.survival.hunger_decay / tick_rate;
        if player.hunger < 0.0 {
            player.hunger = 0.0;
        }
        if player.hunger == 0.0 {
            player.health -= config.survival.starve_damage / tick_rate;
        }
        if player.hunger > config.survival.heal_threshold {
            player.health += config.survival.heal_rate / tick_rate;
        }
        let biome_modifier = world
            .chunks
            .get(&crate::game::world::coords::ChunkCoord::new(
                player.chunk_x,
                player.chunk_y,
            ))
            .and_then(|chunk| {
                config
                    .biomes
                    .biomes
                    .iter()
                    .find(|b| b.id == chunk.biome_id)
                    .map(|b| b.temperature_modifier)
            })
            .unwrap_or(0.0);
        let target_temp = config.survival.neutral_temp + biome_modifier;
        player.temperature += (target_temp - player.temperature) * (1.0 / tick_rate);
        if player.temperature <= 0.0 {
            player.health -= config.survival.freeze_damage / tick_rate;
        }
        if config.survival.thirst_enabled {
            let thirst = player.thirst.get_or_insert(100.0);
            *thirst -= config.survival.thirst_decay / tick_rate;
            if *thirst < 0.0 {
                *thirst = 0.0;
            }
            if *thirst == 0.0 {
                player.health -= config.survival.thirst_damage / tick_rate;
            }
        }
        clamp_health(player, config);
    }
}

pub fn clamp_health(player: &mut PlayerState, config: &Config) {
    if player.health > config.balance.player.max_health {
        player.health = config.balance.player.max_health;
    }
    if player.health < 0.0 {
        player.health = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::game::world::World;
    use crate::game::world::entities::PlayerState;
    use uuid::Uuid;

    #[test]
    fn hunger_decay_reduces_health_when_empty() {
        let config = Config::default();
        let mut world = World::new(config.world.seed);
        let id = Uuid::new_v4();
        let mut player = PlayerState::new(id, Uuid::new_v4(), config.balance.player.inventory_slots);
        player.spawned = true;
        player.hunger = 0.0;
        player.health = 10.0;
        world.players.insert(id, player);
        tick(&mut world, &config, config.server.tick_rate);
        let updated = world.players.get(&id).unwrap();
        assert!(updated.health < 10.0);
    }

}
