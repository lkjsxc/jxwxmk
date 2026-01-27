use uuid::Uuid;

use crate::protocol::client::{AcceptQuestRequest, CraftRequest, NameRequest, NpcActionRequest, SlotRequest, SpawnRequest, SwapSlotsRequest, TradeRequest};
use crate::protocol::client::InputState;

#[derive(Clone, Debug)]
pub struct InputEvent {
    pub player_id: Uuid,
    pub input: InputState,
}

#[derive(Clone, Debug)]
pub struct SpawnEvent {
    pub player_id: Uuid,
    pub request: SpawnRequest,
}

#[derive(Clone, Debug)]
pub struct CraftEvent {
    pub player_id: Uuid,
    pub request: CraftRequest,
}

#[derive(Clone, Debug)]
pub struct TradeEvent {
    pub player_id: Uuid,
    pub request: TradeRequest,
}

#[derive(Clone, Debug)]
pub struct NpcActionEvent {
    pub player_id: Uuid,
    pub request: NpcActionRequest,
}

#[derive(Clone, Debug)]
pub struct AcceptQuestEvent {
    pub player_id: Uuid,
    pub request: AcceptQuestRequest,
}

#[derive(Clone, Debug)]
pub struct SlotEvent {
    pub player_id: Uuid,
    pub request: SlotRequest,
}

#[derive(Clone, Debug)]
pub struct SwapSlotsEvent {
    pub player_id: Uuid,
    pub request: SwapSlotsRequest,
}

#[derive(Clone, Debug)]
pub struct NameEvent {
    pub player_id: Uuid,
    pub request: NameRequest,
}
