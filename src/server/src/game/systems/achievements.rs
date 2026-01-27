use crate::config::Config;
use crate::protocol::{AchievementData, ServerMessage};

use crate::game::messages::OutboundMessage;
use crate::game::world::World;

pub fn evaluate_achievements(world: &mut World, config: &Config, outbox: &mut Vec<OutboundMessage>) {
    for player in world.players.values_mut() {
        let unlocked: Vec<String> = player.achievements.clone();
        for achievement in &config.achievements.achievements {
            if unlocked.iter().any(|id| id == &achievement.id) {
                continue;
            }
            if requirement_met(player, achievement.requirement.kind.as_str(), achievement.requirement.count)
            {
                player.achievements.push(achievement.id.clone());
                player.xp += achievement.reward_xp;
                apply_stat_bonuses(player, &achievement.stat_bonuses);
                let data = AchievementData {
                    id: achievement.id.clone(),
                    name: achievement.name.clone(),
                };
                outbox.push((player.id, ServerMessage::Achievement { data }));
            }
        }
    }
}

fn requirement_met(player: &crate::game::PlayerState, kind: &str, count: u32) -> bool {
    match kind {
        "Steps" => player.stats.steps >= count as u64,
        "Kills" => player.stats.kills >= count as u64,
        "Resources" => player.stats.gathers >= count as u64,
        "Crafts" => player.stats.crafts >= count as u64,
        "Structures" => false,
        "WeaponMastery" => false,
        "PlayerLevel" => player.level >= count,
        "EventParticipation" => false,
        "ReputationTier" => player
            .reputation
            .iter()
            .map(|entry| entry.tier)
            .max()
            .unwrap_or(0)
            >= count,
        _ => false,
    }
}

fn apply_stat_bonuses(player: &mut crate::game::PlayerState, value: &serde_json::Value) {
    if let Some(map) = value.as_object() {
        for (key, val) in map.iter() {
            if let Some(num) = val.as_f64() {
                player.stat_bonuses.push(crate::game::StatBonus {
                    stat: key.clone(),
                    value: num as f32,
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{
        AchievementDef, AchievementRequirement, AchievementsConfig, BalanceConfig, BiomesConfig,
        Config, CraftingConfig, EconomyConfig, QuestsConfig, ServerConfig, SettlementsConfig,
        SpawningConfig, SurvivalConfig, WorldConfig,
    };
    use crate::game::PlayerState;
    use crate::game::world::World;
    use uuid::Uuid;

    fn base_config() -> Config {
        let achievement = AchievementDef {
            id: "first_steps".to_string(),
            name: "First Steps".to_string(),
            description: "Take 1 step".to_string(),
            requirement: AchievementRequirement {
                kind: "Steps".to_string(),
                count: 1,
            },
            reward_xp: 10,
            stat_bonuses: serde_json::json!({ "speed": 0.1 }),
        };
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
            AchievementsConfig {
                achievements: vec![achievement],
            },
        )
    }

    #[test]
    fn awards_xp_on_unlock() {
        let config = base_config();
        let mut world = World::new(0);
        let mut player = PlayerState::new(Uuid::new_v4(), Uuid::new_v4(), 5, 100.0);
        player.spawned = true;
        player.stats.steps = 1;
        world.upsert_player(player);

        let mut outbox = Vec::new();
        evaluate_achievements(&mut world, &config, &mut outbox);
        let player = world.players.values().next().unwrap();
        assert!(player.xp >= 10);
        assert!(player.achievements.contains(&"first_steps".to_string()));
        assert!(!outbox.is_empty());
    }
}
