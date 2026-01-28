use uuid::Uuid;
use world::{Vec2, PlayerState};
use actix::prelude::*;

#[derive(Debug, Clone)]
pub enum GameEvent {
    PlayerJoin {
        player_id: Uuid,
        name: String,
        token: Uuid,
        recipient: Recipient<crate::engine::OutboundMessage>,
    },
    PlayerRejoin {
        state: PlayerState,
        recipient: Recipient<crate::engine::OutboundMessage>,
    },
    PlayerLeave {
        player_id: Uuid,
    },
    Spawn {
        player_id: Uuid,
        settlement_id: Option<Uuid>,
    },
    Input {
        player_id: Uuid,
        dx: f32,
        dy: f32,
        attack: bool,
        interact: bool,
        aim: Option<Vec2>,
    },
    Craft {
        player_id: Uuid,
        recipe_id: String,
    },
    AcceptQuest {
        player_id: Uuid,
        quest_id: String,
    },
    Slot {
        player_id: Uuid,
        slot: u32,
    },
    SwapSlots {
        player_id: Uuid,
        from: u32,
        to: u32,
    },
}
