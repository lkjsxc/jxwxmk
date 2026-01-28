use world::{World, ChunkCoord};
use uuid::Uuid;
use std::collections::HashMap;
use actix::Recipient;
use protocol::{ServerMessage, EntitySnapshot, EntityDeltaData, PlayerUpdateData, InventorySlotProto, QuestState};
use crate::engine::OutboundMessage;

pub fn broadcast_deltas(
    world: &World,
    sessions: &HashMap<Uuid, Recipient<OutboundMessage>>,
    tick_count: u64,
) {
    let mut chunk_players: HashMap<ChunkCoord, Vec<EntitySnapshot>> = HashMap::new();
    for p in world.players.values() {
        if !p.spawned { continue; }
        chunk_players.entry(p.chunk).or_default().push(EntitySnapshot {
            id: p.id.to_string(),
            kind: "player".into(),
            subtype: "player".into(),
            x: p.pos.x,
            y: p.pos.y,
            hp: Some(p.hp),
            max_hp: Some(p.max_hp),
            hunger: Some(p.hunger),
            temp: Some(p.temp),
            level: Some(p.level),
            name: Some(p.name.clone()),
            range: None,
        });
    }

    for (pid, recipient) in sessions {
        if let Some(player) = world.players.get(pid) {
            if !player.spawned { continue; }
            
            // 1. Send Entities (Interest Set) - EVERY TICK
            for &coord in &player.active_view {
                if let Some(players_in_chunk) = chunk_players.get(&coord) {
                    let msg = ServerMessage::EntityDelta {
                        data: EntityDeltaData {
                            chunk: [coord.x, coord.y],
                            updates: players_in_chunk.clone(),
                            removes: vec![],
                        }
                    };
                    if let Ok(json) = serde_json::to_string(&msg) {
                        let _ = recipient.do_send(OutboundMessage(json));
                    }
                }
            }

            // 2. Send Private Player State - REDUCED FREQUENCY (e.g. every 10 ticks or on change)
            // For now, every 10 ticks (3Hz at 30Hz tick rate)
            if tick_count % 10 == 0 {
                let player_update = ServerMessage::PlayerUpdate {
                    data: PlayerUpdateData {
                        inventory: player.inventory.iter().map(|slot| {
                            slot.as_ref().map(|s| InventorySlotProto {
                                item_id: s.item_id.clone(),
                                count: s.count,
                            })
                        }).collect(),
                        active_slot: player.active_slot as u32,
                        xp: player.xp,
                        level: player.level,
                        stats: player.stats.clone(),
                        quests: player.active_quests.iter().map(|q| QuestState {
                            id: q.id.clone(),
                            name: q.id.clone(), 
                            state: format!("{:?}", q.status),
                            objectives: vec![], 
                        }).collect(),
                    }
                };
                if let Ok(json) = serde_json::to_string(&player_update) {
                    let _ = recipient.do_send(OutboundMessage(json));
                }
            }
        }
    }
}