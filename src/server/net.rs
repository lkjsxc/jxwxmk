use serde::{Deserialize, Serialize};
use bincode;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub protocol_version: u32,
    pub msg_type: MessageType,
    pub seq: u64,
    pub payload: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageType {
    Input(InputData),
    Snapshot(SnapshotData),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputData {
    pub player_id: u64,
    pub action: String,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SnapshotData {
    pub server_tick: u64,
    pub world_state: Vec<u8>,
}

#[derive(Debug)]
pub struct InputEvent {
    pub session_id: String,
    pub message: Message,
}

#[derive(Debug)]
pub struct SnapshotEvent {
    pub session_id: String,
    pub message: Message,
}

impl Message {
    pub fn encode(&self) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(self)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(bytes)
    }
}