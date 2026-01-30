use config::{AchievementsConfig, AchievementTemplate};
use world::PlayerState;
use protocol::{AchievementData, NotificationData};

pub struct AchievementSystem;

impl AchievementSystem {
    pub fn check_achievements(
        players: &mut [&mut PlayerState],
        config: &AchievementsConfig,
    ) -> Vec<(uuid::Uuid, AchievementData, NotificationData)> {
        let mut results = Vec::new();

        for player in players.iter_mut().map(|p| &mut **p) {
            for achievement in &config.achievements {
                if player.achievements.contains(&achievement.id) {
                    continue;
                }

                if Self::meets_requirement(player, &achievement.requirement) {
                    player.achievements.push(achievement.id.clone());
                    player.xp += achievement.reward_xp as i64;
                    
                    results.push((
                        player.id,
                        AchievementData {
                            id: achievement.id.clone(),
                            name: achievement.name.clone(),
                        },
                        NotificationData {
                            text: format!("Achievement unlocked: {}", achievement.name),
                        },
                    ));
                }
            }
        }

        results
    }

    fn meets_requirement(player: &PlayerState, requirement: &config::AchievementRequirement) -> bool {
        let stat_value = match requirement.stat.as_str() {
            "steps" => player.stats.steps,
            "kills" => player.stats.kills,
            "crafts" => player.stats.crafts,
            "gathers" => player.stats.gathers,
            "deaths" => player.stats.deaths,
            _ => return false,
        };

        stat_value >= requirement.value
    }
}

// Re-export for use in systems
use config::AchievementRequirement;
