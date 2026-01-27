use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use crate::game::world_state::{Player, ItemType};
use crate::server::protocol::{AchievementData, AchievementRequirement};

pub struct AchievementDef {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub requirement: AchievementRequirement,
    pub stat_bonus: (&'static str, f64),
}

pub fn get_achievement_defs() -> Vec<AchievementDef> {
    vec![
        AchievementDef {
            id: "NoviceWalker",
            name: "Novice Walker",
            description: "Walk 1,000 steps",
            requirement: AchievementRequirement::Steps(1000),
            stat_bonus: ("speed", 0.01),
        },
        AchievementDef {
            id: "FirstBlood",
            name: "First Blood",
            description: "Get your first kill",
            requirement: AchievementRequirement::Kills(1),
            stat_bonus: ("damage", 0.01),
        },
        AchievementDef {
            id: "Lumberjack",
            name: "Lumberjack",
            description: "Gather 100 resources",
            requirement: AchievementRequirement::Gathers(100),
            stat_bonus: ("gather", 0.02),
        },
        // Add more from the list as needed
    ]
}

pub fn check_achievements(player: &mut Player) -> Vec<AchievementData> {
    let defs = get_achievement_defs();
    let mut newly_unlocked = Vec::new();

    for def in defs {
        if player.achievements.contains(def.id) {
            continue;
        }

        let met = match def.requirement {
            AchievementRequirement::Steps(v) => player.stats.steps >= v,
            AchievementRequirement::Kills(v) => player.stats.kills >= v,
            AchievementRequirement::Gathers(v) => player.stats.gathers >= v,
            AchievementRequirement::Crafts(v) => player.stats.crafts >= v,
            AchievementRequirement::Structures(v) => player.stats.structures >= v,
        };

        if met {
            player.achievements.insert(def.id.to_string());
            let (stat, bonus) = def.stat_bonus;
            let current = player.stat_bonuses.entry(stat.to_string()).or_insert(0.0);
            *current += bonus;

            newly_unlocked.push(AchievementData {
                id: def.id.to_string(),
                name: def.name.to_string(),
                description: def.description.to_string(),
                stat_bonus: (stat.to_string(), bonus),
                requirement: def.requirement.clone(),
            });
        }
    }

    newly_unlocked
}
