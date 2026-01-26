use uuid::Uuid;

use crate::world::protocol::ClientInput;

#[derive(Debug, Clone)]
pub struct InputEvent {
    pub session_id: Uuid,
    pub seq: u64,
    pub input: ClientInput,
}

impl InputEvent {
    pub fn new(session_id: Uuid, seq: u64, input: ClientInput) -> Self {
        Self {
            session_id,
            seq,
            input,
        }
    }
}
