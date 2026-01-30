use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Protocol version constant
pub const PROTOCOL_VERSION: u32 = 3;

// Client -> Server messages

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
#[serde(rename_all = "camelCase")]
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
    pub dx: f64,
    pub dy: f64,
    pub attack: bool,
    pub interact: bool,
    pub aim: Option<Aim>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aim {
    pub x: f64,
    pub y: f64,
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
    pub count: i32,
    pub buy: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcActionData {
    pub npc_id: Uuid,
    pub option: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptQuestData {
    pub quest_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotData {
    pub slot: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapSlotsData {
    pub from: usize,
    pub to: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NameData {
    pub name: String,
}

// Server -> Client messages

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
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
    #[serde(rename = "playerUpdate")]
    PlayerUpdate {
        data: PlayerUpdateData,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerUpdateData {
    pub id: Uuid,
    pub name: String,
    pub spawned: bool,
    pub x: f64,
    pub y: f64,
    pub vitals: Vitals,
    pub inventory: Vec<Option<InventorySlot>>,
    pub active_slot: usize,
    pub level: i32,
    pub xp: i64,
    pub stats: PlayerStats,
    pub quests: Vec<QuestInfo>,
    pub achievements: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vitals {
    pub hp: f64,
    pub max_hp: f64,
    pub hunger: f64,
    pub max_hunger: f64,
    pub temperature: f64,
    pub max_temperature: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventorySlot {
    pub item: String,
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerStats {
    pub steps: i64,
    pub kills: i64,
    pub crafts: i64,
    pub gathers: i64,
    pub deaths: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestInfo {
    pub id: String,
    pub name: String,
    pub state: String,
    pub objectives: Vec<QuestObjective>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestObjective {
    pub description: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkAddData {
    pub coord: [i32; 2],
    pub biome: String,
    pub entities: ChunkEntities,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChunkEntities {
    #[serde(default)]
    pub resources: Vec<EntitySnapshot>,
    #[serde(default)]
    pub mobs: Vec<EntitySnapshot>,
    #[serde(default)]
    pub structures: Vec<EntitySnapshot>,
    #[serde(default)]
    pub npcs: Vec<EntitySnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySnapshot {
    pub id: String,
    pub kind: String,
    pub subtype: String,
    pub x: f64,
    pub y: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hp: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_hp: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkRemoveData {
    pub coord: [i32; 2],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityDeltaData {
    pub chunk: [i32; 2],
    pub updates: Vec<EntitySnapshot>,
    pub removes: Vec<EntityRemove>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityRemove {
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub quest: QuestInfo,
}

// HTTP API types

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionClaimRequest {
    pub player_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionClaimResponse {
    pub id: Uuid,
    pub token: Uuid,
}
