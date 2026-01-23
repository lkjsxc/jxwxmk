use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub tick: u64,
    pub players: Vec<crate::models::player::Player>,
    pub world: crate::models::world::World,
}