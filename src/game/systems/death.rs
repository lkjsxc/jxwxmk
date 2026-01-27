use uuid::Uuid;

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

pub fn select_spawn(world: &World, bound_settlement: Option<Uuid>) -> Option<(f32, f32)> {
    let settlement = bound_settlement
        .and_then(|id| world.settlements.get(&id))
        .or_else(|| world.settlements.values().next());
    settlement.map(|settlement| (settlement.spawn_x, settlement.spawn_y))
}

pub fn apply_respawn(player: &mut PlayerState, spawn_x: f32, spawn_y: f32) {
    player.x = spawn_x;
    player.y = spawn_y;
    player.spawned = true;
}
