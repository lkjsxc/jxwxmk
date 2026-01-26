use crate::game::entities::player::Player;
use crate::game::config::MechanicsConfig;

pub struct SurvivalSystem;

impl SurvivalSystem {
    pub fn tick(player: &mut Player, cfg: &MechanicsConfig) {
        let hunger_decay = cfg.hunger_decay / 20.0;
        let heal_rate = cfg.heal_rate / 20.0;
        let starve_dmg = cfg.starve_dmg / 20.0;
        let freeze_dmg = cfg.freeze_dmg / 20.0;

        // Hunger
        player.hunger -= hunger_decay;
        if player.hunger < 0.0 {
            player.hunger = 0.0;
            player.health -= starve_dmg;
        } else if player.hunger > 90.0 && player.health < 100.0 {
            player.health += heal_rate;
        }

        // Temperature
        let target_temp = 50.0; 
        let change_rate = cfg.cold_decay / 20.0;
        if player.cold < target_temp {
            player.cold += change_rate;
        } else {
            player.cold -= change_rate;
        }

        // Freeze Check
        if player.cold <= 0.0 {
            player.health -= freeze_dmg;
        }

        player.health = player.health.clamp(0.0, 100.0);
    }
}