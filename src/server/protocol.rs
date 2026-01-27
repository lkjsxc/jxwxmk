use serde::{Deserialize, Serialize};
use uuid::Uuid;
use actix::prelude::*;
use crate::game::world_state::{World, ItemType};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientMessage {
    pub dx: Option<f64>,
    pub dy: Option<f64>,
    pub attack: Option<bool>,
    pub interact: Option<bool>,
    pub craft: Option<ItemType>,
    pub slot: Option<usize>,
    pub name: Option<String>,
    pub swap_slots: Option<(usize, usize)>,
    pub spawn: Option<bool>,
    pub npc_action: Option<(Uuid, u32)>,
    pub trade: Option<(Uuid, usize, bool)>,
    pub accept_quest: Option<String>,
}

#[derive(Debug, Serialize, Message, Clone)]
#[rtype(result = "()")]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ServerMessage {
    Welcome {
        id: Uuid,
        token: Uuid,
        spawned: bool,
    },
    World {
        data: World,
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

#[derive(Debug, Serialize, Clone)]
pub struct AchievementData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub stat_bonus: (String, f64),
    pub requirement: AchievementRequirement,
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type", content = "value")]
pub enum AchievementRequirement {
    Steps(u64),
    Kills(u64),
    Crafts(u64),
    Gathers(u64),
    Structures(u64),
}

#[derive(Debug, Serialize, Clone)]
pub struct NotificationData {
    pub title: String,
    pub message: String,
    pub color: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct NpcInteractionData {
    pub npc_id: Uuid,
    pub npc_type: String,
    pub name: String,
    pub text: String,
    pub options: Vec<String>,
    pub trade_items: Vec<ItemType>, // Simplified for now
}

#[derive(Debug, Serialize, Clone)]
pub struct QuestUpdateData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub state: String, // "InProgress", "Completed"
    pub objectives: Vec<serde_json::Value>,
}
