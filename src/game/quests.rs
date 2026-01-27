use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::game::world_state::{ItemType, MobType};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum QuestStatus {
    NotStarted,
    InProgress,
    ReadyToTurnIn,
    Completed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Objective {
    Gather {
        item: ItemType,
        count: u32,
        current: u32,
    },
    Kill {
        mob: MobType,
        count: u32,
        current: u32,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuestState {
    pub id: String,
    pub status: QuestStatus,
    pub objectives: Vec<Objective>,
}

#[derive(Debug, Clone)]
pub struct QuestDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub objectives: Vec<Objective>,
    pub reward_text: String,
    // Future: actual rewards
}

pub fn get_quest_definitions() -> HashMap<String, QuestDefinition> {
    let mut map = HashMap::new();

    map.insert("wood_gatherer".to_string(), QuestDefinition {
        id: "wood_gatherer".to_string(),
        name: "Wood Gatherer".to_string(),
        description: "Gather 10 pieces of Wood for the Elder.".to_string(),
        objectives: vec![
            Objective::Gather { item: ItemType::Wood, count: 10, current: 0 }
        ],
        reward_text: "Thanks! Here is some food.".to_string(),
    });

    map.insert("wolf_hunter".to_string(), QuestDefinition {
        id: "wolf_hunter".to_string(),
        name: "Wolf Hunter".to_string(),
        description: "The wolves are dangerous. Eliminate 3 of them.".to_string(),
        objectives: vec![
            Objective::Kill { mob: MobType::Wolf, count: 3, current: 0 }
        ],
        reward_text: "The village is safer now.".to_string(),
    });

    map
}

pub fn check_progress(quest: &mut QuestState, event: &QuestEvent) -> bool {
    if quest.status != QuestStatus::InProgress {
        return false;
    }

    let mut changed = false;
    let mut all_complete = true;

    for obj in &mut quest.objectives {
        let complete = match obj {
            Objective::Gather { item, count, current } => {
                if let QuestEvent::Gather(ev_item, amount) = event {
                    if *ev_item == *item {
                        *current = (*current + *amount).min(*count);
                        changed = true;
                    }
                }
                *current >= *count
            },
            Objective::Kill { mob, count, current } => {
                if let QuestEvent::Kill(ev_mob) = event {
                    if *ev_mob == *mob {
                        *current = (*current + 1).min(*count);
                        changed = true;
                    }
                }
                *current >= *count
            },
        };
        if !complete {
            all_complete = false;
        }
    }

    if all_complete {
        quest.status = QuestStatus::ReadyToTurnIn;
        changed = true;
    }

    changed
}

pub enum QuestEvent {
    Gather(ItemType, u32),
    Kill(MobType),
}
