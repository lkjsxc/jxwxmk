use crate::config::Config;
use crate::game::world::entities::{PlayerQuest, PlayerQuestObjective, PlayerState};
use crate::protocol::server::ServerMessage;
use crate::protocol::types::QuestUpdate;
use uuid::Uuid;

pub fn accept(player: &mut PlayerState, config: &Config, quest_id: &str) -> Option<ServerMessage> {
    let template = config.quests.templates.iter().find(|q| q.id == quest_id)?;
    if player.quests.iter().any(|q| q.id == quest_id) {
        return None;
    }
    let objectives = template
        .objectives
        .iter()
        .map(|obj| PlayerQuestObjective {
            kind: obj.kind.clone(),
            target: obj.target.clone(),
            count: obj.count,
            current: 0,
        })
        .collect();
    let quest = PlayerQuest {
        id: template.id.clone(),
        name: template.name.clone(),
        state: "InProgress".to_string(),
        objectives,
    };
    player.quests.push(quest.clone());
    Some(build_update(player.id, &quest))
}

pub fn apply_gather(player: &mut PlayerState, item: &str, count: u32) -> Vec<ServerMessage> {
    apply_progress(player, "Gather", item, count)
}

pub fn apply_kill(player: &mut PlayerState, mob: &str, count: u32) -> Vec<ServerMessage> {
    apply_progress(player, "Kill", mob, count)
}

pub fn apply_craft(player: &mut PlayerState, recipe: &str, count: u32) -> Vec<ServerMessage> {
    apply_progress(player, "Craft", recipe, count)
}

fn apply_progress(
    player: &mut PlayerState,
    kind: &str,
    target: &str,
    count: u32,
) -> Vec<ServerMessage> {
    let mut messages = Vec::new();
    for quest in &mut player.quests {
        if quest.state != "InProgress" {
            continue;
        }
        let mut updated = false;
        for obj in &mut quest.objectives {
            if obj.kind == kind && obj.target == target {
                obj.current = (obj.current + count).min(obj.count);
                updated = true;
            }
        }
        if updated {
            if quest.objectives.iter().all(|o| o.current >= o.count) {
                quest.state = "ReadyToTurnIn".to_string();
            }
            messages.push(build_update(player.id, quest));
        }
    }
    messages
}

fn build_update(player_id: Uuid, quest: &PlayerQuest) -> ServerMessage {
    let quest_state = crate::protocol::types::QuestState {
        id: quest.id.clone(),
        name: quest.name.clone(),
        state: quest.state.clone(),
        objectives: quest
            .objectives
            .iter()
            .map(|obj| crate::protocol::types::QuestObjective {
                kind: obj.kind.clone(),
                target: obj.target.clone(),
                count: obj.count,
                current: obj.current,
            })
            .collect(),
    };
    let data = QuestUpdate { quest: quest_state };
    let _ = player_id;
    ServerMessage::QuestUpdate { data }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use uuid::Uuid;

    #[test]
    fn quest_progress_updates() {
        let config = Config::default();
        let mut player = PlayerState::new(Uuid::new_v4(), Uuid::new_v4(), 10);
        let msg = accept(&mut player, &config, "caravan_guard");
        assert!(msg.is_some());
        let updates = apply_kill(&mut player, "wolf", 3);
        assert!(!updates.is_empty());
        assert_eq!(player.quests[0].state, "ReadyToTurnIn");
    }
}
