use bincode;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Message {
    pub protocol_version: u16,
    pub msg_type: MessageType,
    pub seq: u32,
    pub payload: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MessageType {
    Input(InputData),
    Snapshot(SnapshotData),
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InputData {
    pub player_id: String,
    pub action: String,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SnapshotData {
    pub tick: u64,
    pub players: Vec<(String, f32, f32, f32, f32)>, // id, x, y, health, hunger
    pub resources: Vec<(u32, String, f32, f32, bool)>, // id, type, x, y, depleted
}

impl Message {
    pub fn encode(&self) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(self)
    }

    pub fn decode(data: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(data)
    }
}

pub type InputEvent = crate::world::InputEvent;
pub type SnapshotEvent = crate::world::SnapshotEvent;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_encode_decode() {
        let msg = Message {
            protocol_version: 1,
            msg_type: MessageType::Input(InputData {
                player_id: "123".to_string(),
                action: "move".to_string(),
                data: vec![1, 2, 3],
            }),
            seq: 42,
            payload: vec![],
        };
        let encoded = msg.encode().unwrap();
        let decoded = Message::decode(&encoded).unwrap();
        assert_eq!(msg, decoded);
    }
}