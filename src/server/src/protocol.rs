use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ClientMessage {
    #[serde(rename = "input")]
    Input(InputPayload),
    #[serde(rename = "spawn")]
    Spawn(SpawnPayload),
    #[serde(rename = "craft")]
    Craft(CraftPayload),
    #[serde(rename = "trade")]
    Trade(TradePayload),
    #[serde(rename = "npcAction")]
    NpcAction(NpcActionPayload),
    #[serde(rename = "acceptQuest")]
    AcceptQuest(AcceptQuestPayload),
    #[serde(rename = "slot")]
    Slot(SlotPayload),
    #[serde(rename = "swapSlots")]
    SwapSlots(SwapSlotsPayload),
    #[serde(rename = "name")]
    Name(NamePayload),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPayload {
    pub dx: f32,
    pub dy: f32,
    pub attack: bool,
    pub interact: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnPayload {
    pub settlement_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftPayload {
    pub recipe: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradePayload {
    pub npc_id: String,
    pub item: String,
    pub count: u32,
    pub buy: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcActionPayload {
    pub npc_id: String,
    pub option: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptQuestPayload {
    pub quest_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotPayload {
    pub slot: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapSlotsPayload {
    pub from: usize,
    pub to: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamePayload {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    #[serde(rename = "welcome")]
    Welcome {
        id: Uuid,
        token: Uuid,
        version: u32,
        spawned: bool,
    },
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
    #[serde(rename = "npcInteraction")]
    NpcInteraction { data: NpcInteractionData },
    #[serde(rename = "questUpdate")]
    QuestUpdate { data: QuestUpdateData },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkAddData {
    pub coord: [i32; 2],
    pub biome: String,
    pub entities: ChunkEntities,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkRemoveData {
    pub coord: [i32; 2],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityDeltaData {
    pub chunk: [i32; 2],
    pub updates: Vec<EntitySnapshot>,
    pub removes: Vec<EntityRemoval>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChunkEntities {
    pub resources: std::collections::HashMap<String, EntitySnapshot>,
    pub mobs: std::collections::HashMap<String, EntitySnapshot>,
    pub structures: std::collections::HashMap<String, EntitySnapshot>,
    pub npcs: std::collections::HashMap<String, EntitySnapshot>,
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
pub struct NpcInteractionData {
    pub npc_id: String,
    pub name: String,
    pub text: String,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestUpdateData {
    pub quest: PlayerQuestPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerQuestPayload {
    pub id: String,
    pub name: String,
    pub state: String,
    pub objectives: Vec<PlayerQuestObjectivePayload>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerQuestObjectivePayload {
    pub kind: String,
    pub count: u32,
    pub current: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub level: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityRemoval {
    pub id: String,
    pub kind: String,
}
impl actix::Message for ServerMessage {
    type Result = ();
}
