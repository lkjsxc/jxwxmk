use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementsConfig {
    pub achievements: Vec<AchievementDef>,
}

impl Default for AchievementsConfig {
    fn default() -> Self {
        Self { achievements: Vec::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub requirement: AchievementRequirement,
    pub reward_xp: i64,
    pub stat_bonuses: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementRequirement {
    pub kind: String,
    pub count: u32,
}
