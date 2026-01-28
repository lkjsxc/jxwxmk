use world::World;
use config::AchievementRequirement;
use std::time::Duration;

pub fn tick(world: &mut World, _dt: Duration) {
    let achievement_configs = world.config.achievements.achievements.clone();
    
    let player_ids: Vec<uuid::Uuid> = world.players.keys().cloned().collect();

    for pid in player_ids {
        for ach_cfg in &achievement_configs {
            let mut unlocked = false;
            
            // Re-scope because we need mutable access to player later
            {
                let player = world.players.get(&pid).unwrap();
                if player.unlocked_achievements.contains(&ach_cfg.id) {
                    continue;
                }

                // Check Requirement
                match &ach_cfg.requirement {
                    AchievementRequirement::Steps { count } => {
                        let current_steps = player.stats.get("steps").cloned().unwrap_or(0.0);
                        if current_steps >= *count as f32 {
                            unlocked = true;
                        }
                    }
                }
            }

            if unlocked {
                if let Some(player) = world.players.get_mut(&pid) {
                    player.unlocked_achievements.insert(ach_cfg.id.clone());
                    player.xp += ach_cfg.rewards.xp as u64;
                    
                    // Apply stat bonuses
                    for (stat, bonus) in &ach_cfg.rewards.stat_bonuses {
                        *player.stat_bonuses.entry(stat.clone()).or_insert(0.0) += bonus;
                    }
                    
                    world.pending_notifications.push((pid, format!("Unlocked: {}", ach_cfg.name)));
                    println!("Player {} unlocked achievement: {}", pid, ach_cfg.name);
                }
            }
        }
    }
}
