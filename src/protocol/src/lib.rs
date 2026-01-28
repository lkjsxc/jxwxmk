use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

// NewTypes for IDs
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct QuestId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AchievementId(pub String);

// --- Client -> Server Messages ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ClientMessage {
    #[serde(rename = "input")]
    Input(InputData),
    #[serde(rename = "spawn")]
    Spawn(SpawnData),
    #[serde(rename = "craft")]
    Craft(CraftData),
    #[serde(rename = "trade")]
    Trade(TradeData),
    #[serde(rename = "npcAction")]
    NpcAction(NpcActionData),
    #[serde(rename = "acceptQuest")]
    AcceptQuest(AcceptQuestData),
    #[serde(rename = "slot")]
    Slot(SlotData),
    #[serde(rename = "swapSlots")]
    SwapSlots(SwapSlotsData),
    #[serde(rename = "name")]
    Name(NameData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputData {
    pub dx: f32,
    pub dy: f32,
    pub attack: bool,
    pub interact: bool,
    pub aim: Option<Vec2>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnData {
    pub settlement_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftData {
    pub recipe: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeData {
    pub npc_id: Uuid,
    pub item: String,
    pub count: u32,
    pub buy: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcActionData {
    pub npc_id: Uuid,
    pub option: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptQuestData {
    pub quest_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotData {
    pub slot: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapSlotsData {
    pub from: u32,
    pub to: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NameData {
    pub name: String,
}


// --- Server -> Client Messages ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    #[serde(rename = "welcome")]
    Welcome { id: Uuid, token: Uuid, version: u32, spawned: bool },
    #[serde(rename = "sessionRevoked")]
    SessionRevoked { reason: String },
    #[serde(rename = "chunkAdd")]
    ChunkAdd { data: ChunkAddData },
    #[serde(rename = "chunkRemove")]
    ChunkRemove { data: ChunkRemoveData },
    #[serde(rename = "entityDelta")]
    EntityDelta { data: EntityDeltaData },
    #[serde(rename = "achievement")]
    Achievement { data: AchievementData },
    #[serde(rename = "notification")]
    Notification { data: NotificationData },
    #[serde(rename = "error")]
    Error { data: ErrorData },
    #[serde(rename = "npcInteraction")]
    NpcInteraction { data: NpcInteractionData },
    #[serde(rename = "questUpdate")]
    QuestUpdate { data: QuestUpdateData },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkAddData {
    pub coord: (i32, i32),
    pub biome: String,
    pub entities: ChunkEntities,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkEntities {
    pub resources: HashMap<String, EntitySnapshot>,
    pub mobs: HashMap<String, EntitySnapshot>,
    pub structures: HashMap<String, EntitySnapshot>,
    pub npcs: HashMap<String, EntitySnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySnapshot {
    pub id: String,
    pub kind: String,
    pub subtype: String,
    pub x: f32,
    pub y: f32,
    pub hp: Option<f32>,
    pub max_hp: Option<f32>,
    pub level: Option<u32>,
    pub name: Option<String>,
    pub range: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkRemoveData {
    pub coord: (i32, i32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityDeltaData {
    pub chunk: (i32, i32),
    pub updates: Vec<EntitySnapshot>,
    pub removes: Vec<EntityRemoval>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityRemoval {
    pub id: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementData {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationData {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorData {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcInteractionData {
    pub npc_id: Uuid,
    pub name: String,
    pub text: String,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestUpdateData {
    pub quest: QuestState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestState {
    pub id: String,
    pub name: String,
    pub state: String, // "InProgress", "Completed"
    pub objectives: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_deserialization() {
        let json = r#"{"type": "input", "data": { "dx": -1.0, "dy": 0.0, "attack": false, "interact": false, "aim": { "x": 12.5, "y": 9.0 } } }"#;
        let msg: ClientMessage = serde_json::from_str(json).unwrap();
        match msg {
            ClientMessage::Input(data) => {
                assert_eq!(data.dx, -1.0);
                assert!(data.aim.is_some());
                assert_eq!(data.aim.unwrap().x, 12.5);
            }
            _ => panic!("Wrong message type"),
        }
    }

    #[test]
    fn test_welcome_serialization() {
        let msg = ServerMessage::Welcome {
            id: Uuid::nil(),
            token: Uuid::nil(),
            version: 3,
            spawned: false,
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("welcome"));
        assert!(json.contains("version"));
    }
}