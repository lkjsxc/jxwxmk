use crate::config::Config;
use crate::protocol::{PlayerQuestObjectivePayload, PlayerQuestPayload, QuestUpdateData, ServerMessage};

use crate::game::entities::{PlayerId, PlayerQuest, PlayerQuestObjective};
use crate::game::messages::OutboundMessage;
use crate::game::world::World;

pub fn accept_quest(
    world: &mut World,
    config: &Config,
    player_id: PlayerId,
    quest_id: &str,
    outbox: &mut Vec<OutboundMessage>,
) {
    let Some(template) = config.quests.templates.iter().find(|q| q.id == quest_id) else {
        return;
    };
    let Some(player) = world.get_player_mut(&player_id) else {
        return;
    };

    if player.quests.iter().any(|q| q.id == quest_id) {
        return;
    }

    let objectives = template
        .objectives
        .iter()
        .map(|obj| PlayerQuestObjective {
            kind: obj.kind.clone(),
            target: obj.target.clone(),
            count: obj.count.unwrap_or(0),
            current: 0,
        })
        .collect();

    player.quests.push(PlayerQuest {
        id: template.id.clone(),
        name: template.name.clone(),
        description: template.description.clone(),
        state: "InProgress".to_string(),
        objectives,
    });

    send_quest_update(player_id, player.quests.last().unwrap(), outbox);
}

pub fn tick_quests(world: &mut World, outbox: &mut Vec<OutboundMessage>) {
    for player in world.players.values_mut() {
        let mut updated = Vec::new();
        for quest in &mut player.quests {
            let mut changed = false;
            for objective in &mut quest.objectives {
                let desired = objective.count;
                let current = match objective.kind.as_str() {
                    "Gather" => player.stats.gathers.min(desired as u64) as u32,
                    "Kill" => player.stats.kills.min(desired as u64) as u32,
                    "Craft" => player.stats.crafts.min(desired as u64) as u32,
                    _ => objective.current,
                };
                if current != objective.current {
                    objective.current = current;
                    changed = true;
                }
            }
            if quest.objectives.iter().all(|o| o.current >= o.count) {
                if quest.state != "ReadyToTurnIn" && quest.state != "Completed" {
                    quest.state = "ReadyToTurnIn".to_string();
                    changed = true;
                }
            }
            if changed {
                updated.push(quest.clone());
            }
        }

        for quest in updated {
            send_quest_update(player.id, &quest, outbox);
        }
    }
}

fn send_quest_update(player_id: PlayerId, quest: &PlayerQuest, outbox: &mut Vec<OutboundMessage>) {
    let payload = PlayerQuestPayload {
        id: quest.id.clone(),
        name: quest.name.clone(),
        state: quest.state.clone(),
        objectives: quest
            .objectives
            .iter()
            .map(|obj| PlayerQuestObjectivePayload {
                kind: obj.kind.clone(),
                count: obj.count,
                current: obj.current,
            })
            .collect(),
    };
    let data = QuestUpdateData { quest: payload };
    outbox.push((player_id, ServerMessage::QuestUpdate { data }));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{
        AchievementsConfig, BalanceConfig, BiomesConfig, Config, CraftingConfig, EconomyConfig,
        QuestObjectiveTemplate, QuestTemplate, QuestsConfig, ServerConfig, SettlementsConfig,
        SpawningConfig, SurvivalConfig, WorldConfig,
    };
    use crate::game::entities::PlayerState;
    use crate::game::world::World;
    use uuid::Uuid;

    fn base_config() -> Config {
        let quest = QuestTemplate {
            id: "caravan_guard".to_string(),
            name: "Guard".to_string(),
            description: "Test".to_string(),
            objectives: vec![QuestObjectiveTemplate {
                kind: "Kill".to_string(),
                target: Some("wolf".to_string()),
                count: Some(1),
            }],
            reward_xp: 10,
        };
        Config::new(
            ServerConfig::default(),
            WorldConfig::default(),
            BalanceConfig::default(),
            SurvivalConfig::default(),
            CraftingConfig::default(),
            SpawningConfig::default(),
            BiomesConfig::default(),
            SettlementsConfig::default(),
            EconomyConfig::default(),
            QuestsConfig {
                templates: vec![quest],
            },
            AchievementsConfig::default(),
        )
    }

    #[test]
    fn accepts_and_updates_quest() {
        let config = base_config();
        let mut world = World::new(0);
        let mut player = PlayerState::new(Uuid::new_v4(), Uuid::new_v4(), 5, 100.0);
        player.spawned = true;
        world.upsert_player(player);
        let player_id = *world.players.keys().next().unwrap();

        let mut outbox = Vec::new();
        accept_quest(&mut world, &config, player_id, "caravan_guard", &mut outbox);
        assert!(!world.players.get(&player_id).unwrap().quests.is_empty());

        world.players.get_mut(&player_id).unwrap().stats.kills = 1;
        tick_quests(&mut world, &mut outbox);
        let quest = &world.players.get(&player_id).unwrap().quests[0];
        assert_eq!(quest.state, "ReadyToTurnIn");
    }
}
