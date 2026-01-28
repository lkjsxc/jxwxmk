use world::{World, QuestStatus, ActiveQuest, QuestObjectiveProgress};
use uuid::Uuid;

pub fn accept_quest(world: &mut World, player_id: Uuid, quest_id: &str) -> Result<(), String> {
    let template = world.config.quests.templates.iter().find(|t| t.id == quest_id)
        .ok_or_else(|| "Quest not found".to_string())?;

    let player = world.players.get_mut(&player_id)
        .ok_or_else(|| "Player not found".to_string())?;

    if player.active_quests.iter().any(|q| q.id == quest_id) {
        return Err("Quest already active".to_string());
    }

    let active = ActiveQuest {
        id: quest_id.to_string(),
        status: QuestStatus::InProgress,
        objectives: template.objectives.iter().map(|obj| {
            match obj {
                config::QuestObjective::Kill { mob_type, count } => QuestObjectiveProgress {
                    objective_type: "kill".to_string(),
                    target: mob_type.clone(),
                    required: *count,
                    current: 0,
                }
            }
        }).collect(),
    };

    player.active_quests.push(active);
    Ok(())
}

pub fn update_quests(world: &mut World, player_id: Uuid, event_type: &str, target: &str, amount: u32) {
    if let Some(player) = world.players.get_mut(&player_id) {
        for quest in player.active_quests.iter_mut() {
            if quest.status != QuestStatus::InProgress { continue; }

            let mut changed = false;
            for obj in quest.objectives.iter_mut() {
                if obj.objective_type == event_type && obj.target == target {
                    obj.current = (obj.current + amount).min(obj.required);
                    changed = true;
                }
            }

            if changed {
                let all_done = quest.objectives.iter().all(|obj| obj.current >= obj.required);
                if all_done {
                    quest.status = QuestStatus::ReadyToTurnIn;
                }
            }
        }
    }
}