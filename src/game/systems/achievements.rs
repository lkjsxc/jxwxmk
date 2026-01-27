use crate::config::Config;
use crate::game::world::entities::PlayerState;
use crate::game::world::World;
use crate::protocol::server::{AchievementData, ServerMessage};
use uuid::Uuid;

pub fn evaluate(world: &mut World, config: &Config) -> Vec<(Uuid, ServerMessage)> {
    let mut messages = Vec::new();

    for player in world.players.values_mut() {
        if !player.spawned {
            continue;
        }
        for achievement in &config.achievements.achievements {
            if player.achievements.contains(&achievement.id) {
                continue;
            }
            if meets_requirement(player, &achievement.requirement.kind, achievement.requirement.count)
            {
                player.achievements.insert(achievement.id.clone());
                player.xp += achievement.reward_xp as u64;
                for (stat, bonus) in &achievement.stat_bonuses {
                    let entry = player.stats.stat_bonuses.entry(stat.clone()).or_insert(0.0);
                    *entry += bonus;
                }
                let data = AchievementData {
                    id: achievement.id.clone(),
                    name: achievement.name.clone(),
                };
                messages.push((player.id, ServerMessage::Achievement { data }));
            }
        }
    }

    messages
}

fn meets_requirement(player: &PlayerState, kind: &str, count: u32) -> bool {
    match kind {
        "Steps" => player.stats.steps >= count as u64,
        "Kills" => player.stats.kills >= count as u64,
        "Resources" => player.stats.gathers >= count as u64,
        "Crafts" => player.stats.crafts >= count as u64,
        "PlayerLevel" => player.level >= count,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::game::world::World;
    use uuid::Uuid;

    #[test]
    fn steps_achievement_unlocks() {
        let config = Config::default();
        let mut world = World::new(config.world.seed);
        let id = Uuid::new_v4();
        let mut player = PlayerState::new(id, Uuid::new_v4(), config.balance.player.inventory_slots);
        player.spawned = true;
        player.stats.steps = 200;
        world.players.insert(id, player);
        let messages = evaluate(&mut world, &config);
        assert!(!messages.is_empty());
    }
}
