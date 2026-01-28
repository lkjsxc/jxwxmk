use world::{World, PlayerState};
use std::time::Duration;

pub fn tick(
    world: &mut World, 
    dt: Duration,
) {
    let dt_secs = dt.as_secs_f32();
    let survival_cfg = world.config.survival.clone();
    let balance_cfg = world.config.balance.clone();
    
    let biome_modifiers: std::collections::HashMap<String, (f32, f32)> = world.config.biomes.biomes.iter()
        .map(|b| (b.id.clone(), (b.hunger_modifier, b.temperature_modifier)))
        .collect();

    let mut notifications = Vec::new();

    for player in world.players.values_mut() {
        let (hunger_mod, temp_mod) = biome_modifiers.get(&player.get_current_biome_id())
            .cloned()
            .unwrap_or((1.0, 0.0));

        let old_hunger = player.hunger;
        let old_thirst = player.thirst;
        let old_temp = player.temp;

        // 1. Hunger Decay
        let hunger_decay = (survival_cfg.hunger_decay * hunger_mod) * dt_secs;
        player.hunger = (player.hunger - hunger_decay).max(0.0);

        // 2. Thirst Decay
        if survival_cfg.thirst_enabled {
            let thirst_decay = survival_cfg.thirst_decay * dt_secs;
            player.thirst = (player.thirst - thirst_decay).max(0.0);
        }

        // 3. Temperature Convergence
        let target_temp = survival_cfg.neutral_temp + temp_mod;
        let converge_rate = survival_cfg.temperature_converge_rate;
        player.temp += (target_temp - player.temp) * (converge_rate * dt_secs);

        // 4. Health logic
        if player.hunger <= 0.0 {
            player.hp -= survival_cfg.starve_damage * dt_secs;
        } else if player.hunger >= survival_cfg.heal_threshold {
            player.hp += survival_cfg.heal_rate * dt_secs;
        }

        if survival_cfg.thirst_enabled && player.thirst <= 0.0 {
            player.hp -= survival_cfg.dehydrate_damage * dt_secs;
        }

        if player.temp <= 0.0 {
            player.hp -= survival_cfg.freeze_damage * dt_secs;
        }

        // Clamp
        player.hp = player.hp.clamp(0.0, balance_cfg.player.max_health);
        player.hunger = player.hunger.clamp(0.0, 100.0);
        player.thirst = player.thirst.clamp(0.0, 100.0);
        player.temp = player.temp.clamp(0.0, 100.0);

        // Notifications
        if old_hunger > 0.0 && player.hunger <= 0.0 {
            notifications.push((player.id, "You are starving!".to_string()));
        }
        if survival_cfg.thirst_enabled && old_thirst > 0.0 && player.thirst <= 0.0 {
            notifications.push((player.id, "You are dehydrated!".to_string()));
        }
        if old_temp > 10.0 && player.temp <= 10.0 {
            notifications.push((player.id, "You are freezing!".to_string()));
        }
    }

    world.pending_notifications.extend(notifications);
}

trait PlayerSurvivalExt {
    fn get_current_biome_id(&self) -> String;
}

impl PlayerSurvivalExt for PlayerState {
    fn get_current_biome_id(&self) -> String {
        "forest".to_string() 
    }
}