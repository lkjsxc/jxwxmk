use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "camelCase")]
pub enum ClientMessage {
    Input(InputState),
    Spawn(SpawnRequest),
    Craft(CraftRequest),
    Trade(TradeRequest),
    NpcAction(NpcActionRequest),
    AcceptQuest(AcceptQuestRequest),
    Slot(SlotRequest),
    SwapSlots(SwapSlotsRequest),
    Name(NameRequest),
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InputState {
    pub dx: f32,
    pub dy: f32,
    pub attack: bool,
    pub interact: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpawnRequest {
    pub settlement_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CraftRequest {
    pub recipe: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeRequest {
    pub npc_id: String,
    pub item: String,
    pub count: u32,
    pub buy: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NpcActionRequest {
    pub npc_id: String,
    pub option: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptQuestRequest {
    pub quest_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlotRequest {
    pub slot: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapSlotsRequest {
    pub from: usize,
    pub to: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NameRequest {
    pub name: String,
}
