use actix::prelude::*;
use uuid::Uuid;
use crate::protocol::ClientMessage;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientConnected {
    pub id: Uuid,
    pub addr: Recipient<crate::protocol::ServerMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientDisconnected {
    pub id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientRequest {
    pub id: Uuid,
    pub msg: ClientMessage,
}
