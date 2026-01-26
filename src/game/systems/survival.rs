use crate::game::entities::player::Player;
use crate::game::config::AppConfig;

pub struct SurvivalSystem;

impl SurvivalSystem {
    pub fn tick(player: &mut Player, cfg: &AppConfig) {
        let tick_rate = cfg.server.tick_rate as f64;
        let hunger_decay = cfg.mechanics.hunger_decay / tick_rate;
        let heal_rate = cfg.mechanics.heal_rate / tick_rate;
        let starve_dmg = cfg.mechanics.starve_dmg / tick_rate;
        let freeze_dmg = cfg.mechanics.freeze_dmg / tick_rate;

        // Hunger
        player.hunger -= hunger_decay;
        if player.hunger < 0.0 {
            player.hunger = 0.0;
            player.health -= starve_dmg;
        } else if player.hunger > cfg.balance.player.heal_threshold && player.health < cfg.balance.player.max_health {
            player.health += heal_rate;
        }

        // Temperature
        let target_temp = cfg.balance.player.neutral_temp; 
        let change_rate = cfg.mechanics.cold_decay / tick_rate;
        if player.cold < target_temp {
            player.cold += change_rate;
        } else {
            player.cold -= change_rate;
        }

        // Freeze Check
        if player.cold <= 0.0 {
            player.health -= freeze_dmg;
        }

        player.health = player.health.clamp(0.0, cfg.balance.player.max_health);
    }
}