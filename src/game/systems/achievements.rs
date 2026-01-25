use crate::game::entities::player::Player;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AchievementId {
    NoviceWalker, MarathonRunner,
    FirstBlood, MonsterHunter, Slayer,
    Lumberjack, Deforestation,
    Miner, DeepDriller,
    ApprenticeSmith, MasterSmith,
    Builder, Architect,
    SeasonedVeteran, LegendarySmith,
    Pacifist, ResourceTycoon,
}

// ...

impl AchievementSystem {
    pub fn get_all() -> Vec<Achievement> {
        vec![
            // ... existing ...
            Achievement { id: AchievementId::NoviceWalker, name: "Novice Walker".into(), description: "Walk 1,000 steps".into(), stat_bonus: ("speed".into(), 0.01) },
            Achievement { id: AchievementId::MarathonRunner, name: "Marathon Runner".into(), description: "Walk 100,000 steps".into(), stat_bonus: ("speed".into(), 0.05) },
            Achievement { id: AchievementId::FirstBlood, name: "First Blood".into(), description: "Kill 1 mob".into(), stat_bonus: ("damage".into(), 0.01) },
            Achievement { id: AchievementId::MonsterHunter, name: "Monster Hunter".into(), description: "Kill 100 mobs".into(), stat_bonus: ("damage".into(), 0.02) },
            Achievement { id: AchievementId::Slayer, name: "Slayer".into(), description: "Kill 1,000 mobs".into(), stat_bonus: ("damage".into(), 0.05) },
            Achievement { id: AchievementId::Lumberjack, name: "Lumberjack".into(), description: "Chop 100 trees".into(), stat_bonus: ("gather".into(), 0.02) },
            Achievement { id: AchievementId::Deforestation, name: "Deforestation".into(), description: "Chop 1,000 trees".into(), stat_bonus: ("gather".into(), 0.05) },
            Achievement { id: AchievementId::Miner, name: "Miner".into(), description: "Mine 100 rocks".into(), stat_bonus: ("gather".into(), 0.02) },
            Achievement { id: AchievementId::DeepDriller, name: "Deep Driller".into(), description: "Mine 1,000 rocks".into(), stat_bonus: ("gather".into(), 0.05) },
            Achievement { id: AchievementId::ApprenticeSmith, name: "Apprentice Smith".into(), description: "Craft 10 items".into(), stat_bonus: ("craft".into(), 0.02) },
            Achievement { id: AchievementId::MasterSmith, name: "Master Smith".into(), description: "Craft 1,000 items".into(), stat_bonus: ("craft".into(), 0.05) },
            Achievement { id: AchievementId::Builder, name: "Builder".into(), description: "Place 50 structures".into(), stat_bonus: ("max_hp".into(), 5.0) },
            Achievement { id: AchievementId::Architect, name: "Architect".into(), description: "Place 500 structures".into(), stat_bonus: ("max_hp".into(), 20.0) },
            
            Achievement { id: AchievementId::SeasonedVeteran, name: "Seasoned Veteran".into(), description: "Tool Level 5".into(), stat_bonus: ("damage".into(), 0.05) },
            Achievement { id: AchievementId::LegendarySmith, name: "Legendary Smith".into(), description: "Tool Level 10".into(), stat_bonus: ("damage".into(), 0.10) },
            Achievement { id: AchievementId::Pacifist, name: "Pacifist".into(), description: "5,000 steps, 0 kills".into(), stat_bonus: ("max_hp".into(), 10.0) },
            Achievement { id: AchievementId::ResourceTycoon, name: "Resource Tycoon".into(), description: "Gather 5,000 resources".into(), stat_bonus: ("gather".into(), 0.10) },
        ]
    }

    pub fn check(player: &mut Player) -> Vec<Achievement> {
        let mut unlocked = Vec::new();
        let all = Self::get_all();

        // Pre-calculate max tool level
        let max_tool_level = player.inventory.slots.iter().flatten()
            .map(|i| i.level).max().unwrap_or(1);

        for ach in all {
            let key = ach.id.to_string();
            if player.achievements.contains(&key) { continue; }

            let completed = match ach.id {
                AchievementId::NoviceWalker => player.stats.steps_taken >= 1000,
                AchievementId::MarathonRunner => player.stats.steps_taken >= 100000,
                AchievementId::FirstBlood => player.stats.mobs_killed >= 1,
                AchievementId::MonsterHunter => player.stats.mobs_killed >= 100,
                AchievementId::Slayer => player.stats.mobs_killed >= 1000,
                AchievementId::Lumberjack => player.stats.resources_gathered >= 100,
                AchievementId::Deforestation => player.stats.resources_gathered >= 1000,
                AchievementId::Miner => player.stats.resources_gathered >= 100,
                AchievementId::DeepDriller => player.stats.resources_gathered >= 1000,
                AchievementId::ApprenticeSmith => player.stats.items_crafted >= 10,
                AchievementId::MasterSmith => player.stats.items_crafted >= 1000,
                AchievementId::Builder => player.stats.structures_placed >= 50,
                AchievementId::Architect => player.stats.structures_placed >= 500,
                AchievementId::SeasonedVeteran => max_tool_level >= 5,
                AchievementId::LegendarySmith => max_tool_level >= 10,
                AchievementId::Pacifist => player.stats.steps_taken >= 5000 && player.stats.mobs_killed == 0,
                AchievementId::ResourceTycoon => player.stats.resources_gathered >= 5000,
            };

            if completed {
                player.achievements.insert(key.clone());
                let (stat, bonus) = &ach.stat_bonus;
                *player.stat_bonuses.entry(stat.clone()).or_insert(0.0) += *bonus;
                unlocked.push(ach);
            }
        }
        unlocked
    }
}
