use actix::prelude::*;
use uuid::Uuid;
use serde::Serialize;
use crate::game::state::World;
use crate::game::systems::achievements::Achievement;
use crate::game::entities::item::ItemType;

#[derive(Message, Clone, Serialize)] #[rtype(result = "()")] 
pub enum ServerMessage {
    WorldUpdate(World),
    AchievementUnlocked(Achievement),
    Notification { title: String, message: String, color: String },
}

#[derive(Message)] #[rtype(result = "()")] pub struct Tick;
#[derive(Message)] #[rtype(result = "Option<(String, Uuid)>")] pub struct Join { pub id: Uuid, pub token: Option<String>, pub addr: Recipient<ServerMessage> }
#[derive(Message)] #[rtype(result = "()")] pub struct Leave { pub id: Uuid }
#[derive(Message)] #[rtype(result = "()")] pub struct Input { pub id: Uuid, pub dx: f64, pub dy: f64, pub attack: bool, pub interact: bool }
#[derive(Message)] #[rtype(result = "()")] pub struct Craft { pub id: Uuid, pub item: ItemType }
#[derive(Message)] #[rtype(result = "()")] pub struct SelectSlot { pub id: Uuid, pub slot: usize }
#[derive(Message)] #[rtype(result = "()")] pub struct UpdateName { pub id: Uuid, pub name: String }
#[derive(Message)] #[rtype(result = "()")] pub struct SwapSlots { pub id: Uuid, pub from: usize, pub to: usize }
#[derive(Message)] #[rtype(result = "()")] pub struct Spawn { pub id: Uuid }
