use serde::{Serialize, Deserialize};
use crate::game::entities::item::ItemType;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QuestState {
    NotStarted,
    InProgress,
    ReadyToTurnIn,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ObjectiveType {
    Gather { item: ItemType, count: u32, current: u32 },
    Kill { mob_type: String, count: u32, current: u32 },
    TalkTo { npc_name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quest {
    pub id: String,
    pub name: String,
    pub description: String,
    pub state: QuestState,
    pub objectives: Vec<ObjectiveType>,
}

impl Quest {
    pub fn check_completion(&mut self) -> bool {
        let ready = self.objectives.iter().all(|obj| match obj {
            ObjectiveType::Gather { count, current, .. } => current >= count,
            ObjectiveType::Kill { count, current, .. } => current >= count,
            ObjectiveType::TalkTo { .. } => self.state == QuestState::ReadyToTurnIn || self.state == QuestState::Completed,
        });
        if ready && self.state == QuestState::InProgress {
            self.state = QuestState::ReadyToTurnIn;
        }
        ready
    }
}

pub struct QuestSystem;

impl QuestSystem {
    pub fn update_gather_progress(quests: &mut Vec<Quest>, item: ItemType, amount: u32) -> Vec<Quest> {
        let mut updated = Vec::new();
        for quest in quests.iter_mut() {
            if quest.state != QuestState::InProgress { continue; }
            let mut changed = false;
            for obj in quest.objectives.iter_mut() {
                if let ObjectiveType::Gather { item: target_item, current, .. } = obj {
                    if *target_item == item {
                        *current += amount;
                        changed = true;
                    }
                }
            }
            if changed {
                quest.check_completion();
                updated.push(quest.clone());
            }
        }
        updated
    }

    pub fn update_kill_progress(quests: &mut Vec<Quest>, mob_type: String) -> Vec<Quest> {
        let mut updated = Vec::new();
        for quest in quests.iter_mut() {
            if quest.state != QuestState::InProgress { continue; }
            let mut changed = false;
            for obj in quest.objectives.iter_mut() {
                if let ObjectiveType::Kill { mob_type: target_mob, current, .. } = obj {
                    if *target_mob == mob_type {
                        *current += 1;
                        changed = true;
                    }
                }
            }
            if changed {
                quest.check_completion();
                updated.push(quest.clone());
            }
        }
        updated
    }

    pub fn get_initial_quests() -> Vec<Quest> {
        vec![
            Quest {
                id: "wood_gatherer".to_string(),
                name: "Wood Gatherer".to_string(),
                description: "Collect 10 pieces of wood for the Elder.".to_string(),
                state: QuestState::NotStarted,
                objectives: vec![ObjectiveType::Gather { item: ItemType::Wood, count: 10, current: 0 }],
            },
            Quest {
                id: "wolf_hunter".to_string(),
                name: "Wolf Hunter".to_string(),
                description: "Exterminate 3 Wolves threatening the village.".to_string(),
                state: QuestState::NotStarted,
                objectives: vec![ObjectiveType::Kill { mob_type: "Wolf".to_string(), count: 3, current: 0 }],
            }
        ]
    }
}
