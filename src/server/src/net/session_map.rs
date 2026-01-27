use crate::game::EngineEvent;
use crate::protocol::ClientMessage;

use super::session::GameSession;

pub fn map_message(session: &GameSession, message: ClientMessage) -> Option<EngineEvent> {
    match message {
        ClientMessage::Input(payload) => {
            if payload.dx.abs() > 1.0 || payload.dy.abs() > 1.0 {
                return None;
            }
            Some(EngineEvent::Input {
                player_id: session.player_id,
                input: crate::game::InputState {
                    dx: payload.dx,
                    dy: payload.dy,
                    attack: payload.attack,
                    interact: payload.interact,
                },
            })
        }
        ClientMessage::Spawn(payload) => Some(EngineEvent::Spawn {
            player_id: session.player_id,
            settlement_id: payload.settlement_id,
        }),
        ClientMessage::Craft(payload) => Some(EngineEvent::Craft {
            player_id: session.player_id,
            recipe: payload.recipe,
        }),
        ClientMessage::Trade(payload) => Some(EngineEvent::Trade {
            player_id: session.player_id,
            npc_id: payload.npc_id,
            item: payload.item,
            count: payload.count,
            buy: payload.buy,
        }),
        ClientMessage::NpcAction(payload) => Some(EngineEvent::NpcAction {
            player_id: session.player_id,
            npc_id: payload.npc_id,
            option: payload.option,
        }),
        ClientMessage::AcceptQuest(payload) => Some(EngineEvent::AcceptQuest {
            player_id: session.player_id,
            quest_id: payload.quest_id,
        }),
        ClientMessage::Slot(payload) => {
            if payload.slot >= session.config.balance.player.inventory_slots {
                return None;
            }
            Some(EngineEvent::SelectSlot {
                player_id: session.player_id,
                slot: payload.slot,
            })
        }
        ClientMessage::SwapSlots(payload) => {
            if payload.from >= session.config.balance.player.inventory_slots
                || payload.to >= session.config.balance.player.inventory_slots
            {
                return None;
            }
            Some(EngineEvent::SwapSlots {
                player_id: session.player_id,
                from: payload.from,
                to: payload.to,
            })
        }
        ClientMessage::Name(payload) => {
            if payload.name.trim().is_empty() {
                return None;
            }
            Some(EngineEvent::Name {
                player_id: session.player_id,
                name: payload.name,
            })
        }
    }
}
