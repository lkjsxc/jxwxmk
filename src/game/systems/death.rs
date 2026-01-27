use crate::config::Config;
use crate::game::world::entities::PlayerState;
use crate::game::world::World;

pub fn handle_deaths(world: &mut World, config: &Config) {
    for player in world.players.values_mut() {
        if player.spawned && player.health <= 0.0 {
            player.spawned = false;
            player.stats.deaths += 1;
            player.health = config.balance.player.max_health;
            player.hunger = 100.0;
            player.temperature = config.survival.neutral_temp;
        }
    }
}

pub fn respawn(world: &mut World, player: &mut PlayerState) {
    let settlement = player
        .bound_settlement
        .and_then(|id| world.settlements.get(&id))
        .or_else(|| world.settlements.values().next());

    if let Some(settlement) = settlement {
        player.x = settlement.spawn_x;
        player.y = settlement.spawn_y;
        player.spawned = true;
    }
}
