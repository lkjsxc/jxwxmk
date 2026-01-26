use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct WorldState {
    pub tick: u64,
    pub players: HashMap<Uuid, PlayerState>,
}

#[derive(Debug, Default)]
pub struct PlayerState {
    pub last_seq: u64,
}
