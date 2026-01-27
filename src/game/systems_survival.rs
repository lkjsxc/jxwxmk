use crate::game::world_state::Player;
use crate::config::AppConfig;

pub fn tick_survival(player: &mut Player) {
    if !player.spawned {
        return;
    }

    let config = AppConfig::get();
    let tick_rate = config.server.tick_rate as f64;

    // Hunger decay
    let hunger_decay = config.mechanics.hunger_decay / tick_rate;
    player.hunger = (player.hunger - hunger_decay).max(0.0);

    // Cold decay (placeholder logic, should check biome/time)
    let cold_decay = config.mechanics.cold_decay / tick_rate;
    player.cold = (player.cold + cold_decay).min(100.0);

    // Starvation damage
    if player.hunger <= 0.0 {
        let starve_dmg = config.mechanics.starve_dmg / tick_rate;
        player.health = (player.health - starve_dmg).max(0.0);
    }

    // Freezing damage
    if player.cold >= 100.0 {
        let freeze_dmg = config.mechanics.freeze_dmg / tick_rate;
        player.health = (player.health - freeze_dmg).max(0.0);
    }

    // Natural healing
    if player.hunger >= config.balance.player.heal_threshold && player.health < config.balance.player.max_health && player.health > 0.0 {
        let heal_rate = config.mechanics.heal_rate / tick_rate;
        player.health = (player.health + heal_rate).min(config.balance.player.max_health);
    }

    // Death check
    if player.health <= 0.0 {
        player.spawned = false;
        player.stats.deaths += 1;
        log::info!("Player {} died from survival conditions", player.id);
    }
}
