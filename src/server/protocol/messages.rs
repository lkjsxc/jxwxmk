use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

// --- Client -> Server Messages ---

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct InputData {
    pub dx: f32,
    pub dy: f32,
    pub attack: bool,
    pub interact: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpawnData {
    pub settlement_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CraftData {
    pub recipe: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TradeData {
    pub npc_id: Uuid,
    pub item: String,
    pub count: u32,
    pub buy: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NpcActionData {
    pub npc_id: Uuid,
    pub option: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AcceptQuestData {
    pub quest_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SlotData {
    pub slot: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SwapSlotsData {
    pub from: u32,
    pub to: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NameData {
    pub name: String,
}

// --- Server -> Client Messages ---

#[derive(Debug, Serialize)]
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
    #[serde(rename = "npcInteraction")]
    NpcInteraction {
        data: NpcInteractionData,
    },
    #[serde(rename = "questUpdate")]
    QuestUpdate {
        data: QuestUpdateData,
    },
}

#[derive(Debug, Serialize)]
pub struct ChunkAddData {
    pub coord: (i32, i32),
    pub biome: String,
    pub entities: ChunkEntities,
}

#[derive(Debug, Serialize)]
pub struct ChunkEntities {
    pub resources: HashMap<Uuid, EntitySnapshot>,
    pub mobs: HashMap<Uuid, EntitySnapshot>,
    pub structures: HashMap<Uuid, EntitySnapshot>,
    pub npcs: HashMap<Uuid, EntitySnapshot>,
}

#[derive(Debug, Serialize)]
pub struct ChunkRemoveData {
    pub coord: (i32, i32),
}

#[derive(Debug, Serialize)]
pub struct EntityDeltaData {
    pub chunk: (i32, i32),
    pub updates: Vec<EntitySnapshot>,
    pub removes: Vec<EntityRemove>,
}

#[derive(Debug, Serialize)]
pub struct AchievementData {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct NotificationData {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct NpcInteractionData {
    pub npc_id: Uuid,
    pub name: String,
    pub text: String,
    pub options: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct QuestUpdateData {
    pub quest: QuestSnapshot,
}

#[derive(Debug, Serialize)]
pub struct QuestSnapshot {
    pub id: String,
    pub name: String,
    pub state: String, // NotStarted, InProgress, ReadyToTurnIn, Completed
    pub objectives: Vec<ObjectiveSnapshot>,
}

#[derive(Debug, Serialize)]
pub struct ObjectiveSnapshot {
    pub description: String,
    pub current: u32,
    pub target: u32,
}

#[derive(Debug, Serialize)]
pub struct EntitySnapshot {
    pub id: Uuid,
    pub kind: String, // player, resource, mob, structure, npc
    pub subtype: String,
    pub x: f64,
    pub y: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hp: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_hp: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct EntityRemove {
    pub id: Uuid,
    pub kind: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_input_serialization() {
        let msg = ClientMessage::Input(InputData {
            dx: 1.0,
            dy: 0.0,
            attack: false,
            interact: true,
        });

        let serialized = serde_json::to_string(&msg).unwrap();
        let expected = json!({
            "type": "input",
            "data": {
                "dx": 1.0,
                "dy": 0.0,
                "attack": false,
                "interact": true
            }
        });

        assert_eq!(serde_json::from_str::<serde_json::Value>(&serialized).unwrap(), expected);
    }
}