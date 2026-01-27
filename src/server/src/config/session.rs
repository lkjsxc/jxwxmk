use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionClaimRequest {
    pub player_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionClaimResponse {
    pub id: Uuid,
    pub token: Uuid,
}
