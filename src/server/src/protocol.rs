use bytes::{Buf, BufMut, BytesMut};
use thiserror::Error;
use tracing::{debug, error};

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("Invalid message format")]
    InvalidFormat,
    #[error("Invalid protocol version: {0}")]
    InvalidVersion(u8),
    #[error("Invalid message type: {0}")]
    InvalidMessageType(u8),
    #[error("Message too short")]
    MessageTooShort,
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    Authenticate = 0,
    Input = 1,
    StateUpdate = 2,
    Ping = 3,
    Pong = 4,
    Error = 5,
}

impl From<u8> for MessageType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Authenticate,
            1 => Self::Input,
            2 => Self::StateUpdate,
            3 => Self::Ping,
            4 => Self::Pong,
            5 => Self::Error,
            _ => Self::Error,
        }
    }
}

#[derive(Debug)]
pub struct BinaryMessage {
    pub protocol_version: u8,
    pub message_type: MessageType,
    pub sequence: u32,
    pub payload: Vec<u8>,
}

pub fn parse_binary_message(buffer: &[u8]) -> Result<BinaryMessage, ProtocolError> {
    if buffer.len() < 7 {
        return Err(ProtocolError::MessageTooShort);
    }
    
    let mut cursor = std::io::Cursor::new(buffer);
    
    // Read protocol version (1 byte)
    let protocol_version = cursor.get_u8();
    if protocol_version != 1 {
        return Err(ProtocolError::InvalidVersion(protocol_version));
    }
    
    // Read message type (1 byte)
    let message_type = MessageType::from(cursor.get_u8());
    
    // Read sequence number (4 bytes)
    let sequence = cursor.get_u32();
    
    // Read remaining payload
    let payload_length = buffer.len() - 6;
    let mut payload = vec![0u8; payload_length];
    cursor.read_exact(&mut payload)
        .map_err(|_| ProtocolError::InvalidFormat)?;
    
    Ok(BinaryMessage {
        protocol_version,
        message_type,
        sequence,
        payload,
    })
}

pub fn serialize_binary_message(
    protocol_version: u8,
    message_type: MessageType,
    sequence: u32,
    payload: &[u8],
) -> Result<Vec<u8>, ProtocolError> {
    let mut buffer = BytesMut::with_capacity(6 + payload.len());
    
    // Write protocol version
    buffer.put_u8(protocol_version);
    
    // Write message type
    buffer.put_u8(message_type as u8);
    
    // Write sequence number
    buffer.put_u32(sequence);
    
    // Write payload
    buffer.extend_from_slice(payload);
    
    Ok(buffer.to_vec())
}

pub fn deserialize_authenticate_message(payload: &[u8]) -> Result<String, ProtocolError> {
    String::from_utf8(payload.to_vec())
        .map_err(|e| ProtocolError::SerializationError(e.to_string()))
}

pub fn serialize_authenticated_message(player_id: &str, server_tick: u64) -> Result<Vec<u8>, ProtocolError> {
    let mut buffer = BytesMut::new();
    buffer.extend_from_slice(player_id.as_bytes());
    buffer.put_u8(b'|');
    buffer.put_u64(server_tick);
    Ok(buffer.to_vec())
}

pub fn deserialize_input_message(payload: &[u8]) -> Result<(PlayerInput, u32), ProtocolError> {
    // In a real implementation, this would deserialize the binary input format
    // For now, we'll return a simple default
    Ok((PlayerInput::default(), 0))
}

#[derive(Debug, Default)]
pub struct PlayerInput {
    pub movement: MovementInput,
    pub actions: Vec<PlayerAction>,
}

#[derive(Debug, Default)]
pub struct MovementInput {
    pub direction: (f32, f32),
    pub speed: f32,
    pub sprinting: bool,
}

#[derive(Debug)]
pub enum PlayerAction {
    Attack,
    UseItem { slot: usize },
    Craft { recipe_id: String },
    Interact,
}