use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct QuestsConfig {
    pub templates: Vec<QuestTemplate>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct QuestTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub objectives: Vec<ObjectiveTemplate>,
    pub reward_xp: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ObjectiveTemplate {
    pub kind: String,
    pub target: String,
    pub count: u32,
}

impl Default for QuestsConfig {
    fn default() -> Self {
        Self {
            templates: vec![QuestTemplate {
                id: "caravan_guard".to_string(),
                name: "Guard the Caravan".to_string(),
                description: "Defeat nearby threats.".to_string(),
                objectives: vec![ObjectiveTemplate {
                    kind: "Kill".to_string(),
                    target: "wolf".to_string(),
                    count: 3,
                }],
                reward_xp: 25,
            }],
        }
    }
}

impl Default for QuestTemplate {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            name: "".to_string(),
            description: "".to_string(),
            objectives: Vec::new(),
            reward_xp: 0,
        }
    }
}

impl Default for ObjectiveTemplate {
    fn default() -> Self {
        Self {
            kind: "".to_string(),
            target: "".to_string(),
            count: 0,
        }
    }
}
