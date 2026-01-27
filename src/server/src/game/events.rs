use uuid::Uuid;

use super::entities::PlayerId;

#[derive(Debug, Clone)]
pub struct InputState {
    pub dx: f32,
    pub dy: f32,
    pub attack: bool,
    pub interact: bool,
}

#[derive(Debug, Clone)]
pub enum EngineEvent {
    Input {
        player_id: PlayerId,
        input: InputState,
    },
    Spawn {
        player_id: PlayerId,
        settlement_id: Option<String>,
    },
    Craft {
        player_id: PlayerId,
        recipe: String,
    },
    Trade {
        player_id: PlayerId,
        npc_id: String,
        item: String,
        count: u32,
        buy: bool,
    },
    NpcAction {
        player_id: PlayerId,
        npc_id: String,
        option: u32,
    },
    AcceptQuest {
        player_id: PlayerId,
        quest_id: String,
    },
    SelectSlot {
        player_id: PlayerId,
        slot: usize,
    },
    SwapSlots {
        player_id: PlayerId,
        from: usize,
        to: usize,
    },
    Name {
        player_id: PlayerId,
        name: String,
    },
    Join {
        player_id: PlayerId,
        token: Uuid,
    },
    Leave {
        player_id: PlayerId,
    },
}
