use serde::{Deserialize, Serialize};

pub const PROTOCOL_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "msg_type")]
pub enum ClientMessage {
    Input {
        protocol_version: u32,
        seq: u64,
        input: ClientInput,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInput {
    pub movement: MovementInput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementInput {
    pub dx: i8,
    pub dy: i8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "msg_type")]
pub enum ServerMessage {
    Welcome {
        protocol_version: u32,
        server_tick: u64,
    },
    Snapshot {
        protocol_version: u32,
        server_tick: u64,
    },
    Error {
        protocol_version: u32,
        code: String,
        message: String,
    },
}
