use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

// --- Common Types ---

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
}

// --- Client -> Server Messages ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub struct InputData {
    pub dx: f32,
    pub dy: f32,
    pub attack: bool,
    pub interact: bool,
    pub aim: Option<Vec2>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpawnData {
    pub settlement_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CraftData {
    pub recipe: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TradeData {
    pub npc_id: Uuid,
    pub item: String,
    pub count: u32,
    pub buy: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NpcActionData {
    pub npc_id: Uuid,
    pub option: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AcceptQuestData {
    pub quest_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SlotData {
    pub slot: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SwapSlotsData {
    pub from: u32,
    pub to: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NameData {
    pub name: String,
}

// --- Server -> Client Messages ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")] // Welcome is special, fields are top-level
#[serde(deny_unknown_fields)]
pub enum ServerMessage {
    #[serde(rename = "welcome")]
    Welcome {
        id: Uuid,
        token: Uuid,
        version: u32,
        spawned: bool,
    },
    #[serde(rename = "sessionRevoked")]
    SessionRevoked {
        reason: String,
    },
    #[serde(rename = "chunkAdd")]
    ChunkAdd {
        data: ChunkAddData,
    },
    #[serde(rename = "chunkRemove")]
    ChunkRemove {
        data: ChunkRemoveData,
    },
    #[serde(rename = "entityDelta")]
    EntityDelta {
        data: EntityDeltaData,
    },
    #[serde(rename = "achievement")]
    Achievement {
        data: AchievementData,
    },
    #[serde(rename = "notification")]
    Notification {
        data: NotificationData,
    },
    #[serde(rename = "error")]
    Error {
        data: ErrorData,
    },
    #[serde(rename = "npcInteraction")]
    NpcInteraction {
        data: NpcInteractionData,
    },
    #[serde(rename = "questUpdate")]
    QuestUpdate {
        data: QuestUpdateData,
    },
    #[serde(rename = "playerUpdate")]
    PlayerUpdate {
        data: PlayerUpdateData,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlayerUpdateData {
    pub inventory: Vec<Option<InventorySlotProto>>,
    pub active_slot: u32,
    pub xp: u64,
    pub level: u32,
    pub stats: HashMap<String, f32>,
    pub quests: Vec<QuestState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InventorySlotProto {
    pub item_id: String,
    pub count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChunkAddData {
    pub coord: [i32; 2],
    pub biome: String,
    pub entities: ChunkEntities,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChunkEntities {
    #[serde(default)]
    pub resources: HashMap<String, EntitySnapshot>,
    #[serde(default)]
    pub mobs: HashMap<String, EntitySnapshot>,
    #[serde(default)]
    pub structures: HashMap<String, EntitySnapshot>,
    #[serde(default)]
    pub npcs: HashMap<String, EntitySnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EntitySnapshot {
    pub id: String,
    pub kind: String,
    pub subtype: String,
    pub x: f32,
    pub y: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hp: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_hp: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hunger: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChunkRemoveData {
    pub coord: [i32; 2],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EntityDeltaData {
    pub chunk: [i32; 2],
    pub updates: Vec<EntitySnapshot>,
    pub removes: Vec<EntityRemoval>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EntityRemoval {
    pub id: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AchievementData {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NotificationData {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ErrorData {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NpcInteractionData {
    pub npc_id: Uuid,
    pub name: String,
    pub text: String,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuestUpdateData {
    pub quest: QuestState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuestState {
    pub id: String,
    pub name: String,
    pub state: String, // InProgress, Completed, etc.
    pub objectives: Vec<serde_json::Value>, // Keeping generic for now, strict type if needed
}
