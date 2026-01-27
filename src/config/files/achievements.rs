use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct AchievementsConfig {
    pub achievements: Vec<AchievementDef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct AchievementDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub requirement: RequirementDef,
    pub reward_xp: u32,
    pub stat_bonuses: HashMap<String, f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct RequirementDef {
    pub kind: String,
    pub count: u32,
}

impl Default for AchievementsConfig {
    fn default() -> Self {
        Self {
            achievements: vec![AchievementDef {
                id: "first_steps".to_string(),
                name: "First Steps".to_string(),
                description: "Take 100 steps.".to_string(),
                requirement: RequirementDef {
                    kind: "Steps".to_string(),
                    count: 100,
                },
                reward_xp: 10,
                stat_bonuses: HashMap::new(),
            }],
        }
    }
}

impl Default for AchievementDef {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            name: "".to_string(),
            description: "".to_string(),
            requirement: RequirementDef::default(),
            reward_xp: 0,
            stat_bonuses: HashMap::new(),
        }
    }
}

impl Default for RequirementDef {
    fn default() -> Self {
        Self {
            kind: "".to_string(),
            count: 0,
        }
    }
}
