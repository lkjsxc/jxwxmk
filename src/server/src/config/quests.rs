use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestsConfig {
    pub templates: Vec<QuestTemplate>,
}

impl Default for QuestsConfig {
    fn default() -> Self {
        Self { templates: Vec::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub objectives: Vec<QuestObjectiveTemplate>,
    pub reward_xp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestObjectiveTemplate {
    pub kind: String,
    pub target: Option<String>,
    pub count: Option<u32>,
}
