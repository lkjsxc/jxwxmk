use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub id: Uuid,
    pub seed: u64,
    pub width: u32,
    pub height: u32,
    pub players: Vec<crate::models::player::Player>,
}