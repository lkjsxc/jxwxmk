use actix::Message;
use uuid::Uuid;

use crate::protocol::client::{
    AcceptQuestRequest, CraftRequest, InputState, NameRequest, NpcActionRequest, SlotRequest,
    SpawnRequest, SwapSlotsRequest, TradeRequest,
};

#[derive(Message)]
#[rtype(result = "JoinResult")]
pub struct Join {
    pub session_id: Uuid,
    pub token: Option<Uuid>,
    pub addr: actix::Recipient<crate::protocol::server::ServerMessage>,
}

#[derive(Clone, Debug)]
pub struct JoinResult {
    pub player_id: Uuid,
    pub token: Uuid,
    pub spawned: bool,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Leave {
    pub session_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct InputMsg {
    pub player_id: Uuid,
    pub input: InputState,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SpawnMsg {
    pub player_id: Uuid,
    pub request: SpawnRequest,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct CraftMsg {
    pub player_id: Uuid,
    pub request: CraftRequest,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct TradeMsg {
    pub player_id: Uuid,
    pub request: TradeRequest,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct NpcActionMsg {
    pub player_id: Uuid,
    pub request: NpcActionRequest,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct AcceptQuestMsg {
    pub player_id: Uuid,
    pub request: AcceptQuestRequest,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SlotMsg {
    pub player_id: Uuid,
    pub request: SlotRequest,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SwapSlotsMsg {
    pub player_id: Uuid,
    pub request: SwapSlotsRequest,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct NameMsg {
    pub player_id: Uuid,
    pub request: NameRequest,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RevokePlayer {
    pub player_id: Uuid,
}
