use config::QuestsConfig;
use world::PlayerState;
use protocol::{QuestInfo, QuestUpdateData, QuestObjective};

pub struct QuestSystem;

impl QuestSystem {
    pub fn accept_quest(
        player: &mut PlayerState,
        quest_id: &str,
        config: &QuestsConfig,
    ) -> Option<QuestUpdateData> {
        // Check if already has quest
        if player.quests.iter().any(|q| q.id == quest_id) {
            return None;
        }

        // Find quest template
        let template = config.quests.iter().find(|q| q.id == quest_id)?;

        let quest_info = QuestInfo {
            id: template.id.clone(),
            name: template.name.clone(),
            state: "InProgress".to_string(),
            objectives: template.objectives.iter().map(|o| QuestObjective {
                description: format!("{} {} {}", o.r#type, o.count, o.target),
                completed: false,
            }).collect(),
        };

        player.quests.push(quest_info.clone());

        Some(QuestUpdateData { quest: quest_info })
    }

    pub fn update_quest_progress(
        player: &mut PlayerState,
        quest_id: &str,
        objective_index: usize,
        completed: bool,
    ) -> Option<QuestUpdateData> {
        let quest = player.quests.iter_mut().find(|q| q.id == quest_id)?;
        
        if let Some(objective) = quest.objectives.get_mut(objective_index) {
            objective.completed = completed;
        }

        // Check if all objectives completed
        if quest.objectives.iter().all(|o| o.completed) {
            quest.state = "Completed".to_string();
        }

        Some(QuestUpdateData { quest: quest.clone() })
    }
}
