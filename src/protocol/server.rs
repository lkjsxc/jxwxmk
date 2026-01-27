use serde::{Deserialize, Serialize};

use crate::protocol::types::{ChunkAddData, ChunkRemoveData, EntityDeltaData, QuestUpdate};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ServerMessage {
    Welcome {
        id: String,
        token: String,
        version: u32,
        spawned: bool,
    },
    SessionRevoked {
        reason: String,
    },
    ChunkAdd {
        data: ChunkAddData,
    },
    ChunkRemove {
        data: ChunkRemoveData,
    },
    EntityDelta {
        data: EntityDeltaData,
    },
    Achievement {
        data: AchievementData,
    },
    Notification {
        data: NotificationData,
    },
    NpcInteraction {
        data: NpcInteractionData,
    },
    QuestUpdate {
        data: QuestUpdate,
    },
}

impl actix::Message for ServerMessage {
    type Result = ();
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AchievementData {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationData {
    pub text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NpcInteractionData {
    pub npc_id: String,
    pub name: String,
    pub text: String,
    pub options: Vec<String>,
}
