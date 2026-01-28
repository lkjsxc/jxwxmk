use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    #[serde(rename = "attack")]
    Attack(AttackData),
    #[serde(rename = "gather")]
    Gather(GatherData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InputData {
    pub dx: f32,
    pub dy: f32,
    #[serde(default)]
    pub attack: bool,
    #[serde(default)]
    pub interact: bool,
    pub aim: Option<Aim>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Aim {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpawnData {
    #[serde(default)]
    pub settlement_id: Option<String>,
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
    pub count: i32,
    pub buy: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NpcActionData {
    pub npc_id: Uuid,
    pub option: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AcceptQuestData {
    pub quest_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SlotData {
    pub slot: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SwapSlotsData {
    pub from: usize,
    pub to: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NameData {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AttackData {
    pub target_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GatherData {
    pub resource_id: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    #[serde(rename = "welcome")]
    Welcome {
        id: Uuid,
        token: Uuid,
        version: i32,
        spawned: bool,
    },
    #[serde(rename = "sessionRevoked")]
    SessionRevoked { reason: String },
    #[serde(rename = "playerUpdate")]
    PlayerUpdate { data: PlayerUpdateData },
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
    #[serde(rename = "combatResult")]
    CombatResult { data: CombatResultData },
    #[serde(rename = "resourceDepleted")]
    ResourceDepleted { data: ResourceDepletedData },
}

#[derive(Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PlayerUpdateData {
    pub id: Uuid,
    pub name: String,
    pub spawned: bool,
    pub vitals: Vitals,
    pub inventory: Vec<Option<InventorySlot>>,
    pub active_slot: usize,
    pub level: i32,
    pub xp: i64,
    pub stats: PlayerStats,
    pub quests: Vec<Quest>,
    pub achievements: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Vitals {
    pub hp: f32,
    pub max_hp: f32,
    pub hunger: f32,
    pub max_hunger: f32,
    pub temperature: f32,
    pub max_temperature: f32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InventorySlot {
    pub item: String,
    pub count: i32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PlayerStats {
    pub steps: i64,
    pub kills: i64,
    pub crafts: i64,
    pub gathers: i64,
    pub deaths: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Quest {
    pub id: String,
    pub name: String,
    pub state: String,
    pub objectives: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChunkAddData {
    pub coord: [i32; 2],
    pub biome: String,
    pub entities: ChunkEntities,
}

#[derive(Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChunkEntities {
    pub resources: serde_json::Value,
    pub mobs: serde_json::Value,
    pub structures: serde_json::Value,
    pub npcs: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChunkRemoveData {
    pub coord: [i32; 2],
}

#[derive(Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EntityDeltaData {
    pub chunk: [i32; 2],
    pub updates: Vec<serde_json::Value>,
    pub removes: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AchievementData {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NotificationData {
    pub text: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ErrorData {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NpcInteractionData {
    pub npc_id: Uuid,
    pub name: String,
    pub text: String,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QuestUpdateData {
    pub quest: Quest,
}

#[derive(Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CombatResultData {
    pub target_id: Uuid,
    pub damage: f32,
    pub hit: bool,
    pub critical: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceDepletedData {
    pub resource_id: String,
    pub items_received: Vec<InventorySlot>,
}
