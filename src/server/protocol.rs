use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ClientMessage {
    Input { data: InputData },
    Spawn { data: SpawnData },
    Craft { data: CraftData },
    Trade { data: TradeData },
    NpcAction { data: NpcActionData },
    AcceptQuest { data: AcceptQuestData },
    Slot { data: SlotData },
    SwapSlots { data: SwapSlotsData },
    Name { data: NameData },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputData {
    pub dx: f32,
    pub dy: f32,
    pub attack: bool,
    pub interact: bool,
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
    pub option: usize,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ServerMessage {
    Welcome {
        id: Uuid,
        token: Uuid,
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
        data: QuestUpdateData,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkAddData {
    pub coord: (i32, i32),
    pub biome: String,
    pub entities: ChunkEntities,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkEntities {
    pub resources: HashMap<Uuid, EntitySnapshot>,
    pub mobs: HashMap<Uuid, EntitySnapshot>,
    pub structures: HashMap<Uuid, EntitySnapshot>,
    pub npcs: HashMap<Uuid, EntitySnapshot>,
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
    pub npc_id: Uuid,
    pub name: String,
    pub text: String,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestUpdateData {
    pub quest: QuestSnapshot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestSnapshot {
    pub id: String,
    pub name: String,
    pub state: String,
    pub objectives: Vec<String>, // Simplified for now
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySnapshot {
    pub id: Uuid,
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
    pub id: Uuid,
    pub kind: String,
}
