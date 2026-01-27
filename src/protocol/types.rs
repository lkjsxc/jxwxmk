use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum EntityKind {
    Player,
    Resource,
    Mob,
    Structure,
    Npc,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityUpdate {
    pub id: String,
    pub kind: EntityKind,
    pub subtype: String,
    pub x: f32,
    pub y: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hp: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_hp: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityRemove {
    pub id: String,
    pub kind: EntityKind,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChunkEntities {
    pub resources: BTreeMap<String, EntityUpdate>,
    pub mobs: BTreeMap<String, EntityUpdate>,
    pub structures: BTreeMap<String, EntityUpdate>,
    pub npcs: BTreeMap<String, EntityUpdate>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChunkAddData {
    pub coord: [i32; 2],
    pub biome: String,
    pub entities: ChunkEntities,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChunkRemoveData {
    pub coord: [i32; 2],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityDeltaData {
    pub chunk: [i32; 2],
    pub updates: Vec<EntityUpdate>,
    pub removes: Vec<EntityRemove>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestUpdate {
    pub quest: QuestState,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestState {
    pub id: String,
    pub name: String,
    pub state: String,
    pub objectives: Vec<QuestObjective>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestObjective {
    pub kind: String,
    pub target: String,
    pub count: u32,
    pub current: u32,
}
