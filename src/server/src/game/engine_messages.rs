use actix::Message;
use actix::Recipient;
use uuid::Uuid;

use crate::protocol::ServerMessage;
use crate::game::entities::PlayerId;
use crate::game::events::EngineEvent;

#[derive(Debug, Clone)]
pub struct JoinResult {
    pub id: PlayerId,
    pub token: Uuid,
    pub spawned: bool,
}

#[derive(Message)]
#[rtype(result = "JoinResult")]
pub struct JoinCommand {
    pub player_id: PlayerId,
    pub token: Uuid,
    pub session: Recipient<ServerMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct LeaveCommand {
    pub player_id: PlayerId,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct EngineEventCommand {
    pub event: EngineEvent,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RevokeSession {
    pub player_id: PlayerId,
    pub reason: String,
}
