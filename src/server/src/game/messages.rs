use crate::protocol::ServerMessage;
use crate::game::entities::PlayerId;

pub type OutboundMessage = (PlayerId, ServerMessage);
