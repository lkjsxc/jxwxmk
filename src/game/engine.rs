use actix::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;
use crate::game::state::World;
use crate::game::entities::player::Player;
use crate::game::entities::resource::{Resource, ResourceType};
use crate::game::entities::mob::{Mob, MobType};
use crate::game::entities::item::ItemType;
use crate::game::entities::structure::{Structure, StructureType};
use crate::game::systems::survival::SurvivalSystem;
use crate::game::systems::crafting::CraftingSystem;
use crate::game::config::AppConfig;
use serde::Serialize;
use rand::Rng;

// Messages
#[derive(Message)] #[rtype(result = "()")] pub struct Tick;
#[derive(Message)] #[rtype(result = "Option<(String, Uuid)>")] pub struct Join { pub id: Uuid, pub token: Option<String>, pub addr: Recipient<WorldUpdate> }
#[derive(Message)] #[rtype(result = "()")] pub struct Leave { pub id: Uuid }
#[derive(Message)] #[rtype(result = "()")] pub struct Input { pub id: Uuid, pub dx: f64, pub dy: f64, pub attack: bool, pub interact: bool }
#[derive(Message)] #[rtype(result = "()")] pub struct Craft { pub id: Uuid, pub item: ItemType }
#[derive(Message)] #[rtype(result = "()")] pub struct SelectSlot { pub id: Uuid, pub slot: usize }
#[derive(Message)] #[rtype(result = "()")] pub struct UpdateName { pub id: Uuid, pub name: String }
#[derive(Message, Clone, Serialize)] #[rtype(result = "()")] pub struct WorldUpdate(pub World);

pub struct GameEngine { world: World, sessions: HashMap<Uuid, Recipient<WorldUpdate>>, config: AppConfig }

impl GameEngine {
    pub fn new() -> Self { Self { world: World::new(), sessions: HashMap::new(), config: AppConfig::load() } }
    fn broadcast(&self) { let update = WorldUpdate(self.world.clone()); for addr in self.sessions.values() { addr.do_send(update.clone()); } }
    fn spawn_initial_entities(&mut self) {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let x = rng.gen_range(0.0..self.world.width); let y = rng.gen_range(0.0..self.world.height);
            let r_type = match rng.gen_range(0..10) { 0..=4 => ResourceType::Tree, 5..=8 => ResourceType::Rock, _ => ResourceType::Food };
            let res = Resource::new(r_type, x, y); self.world.resources.insert(res.id, res);
        }
        for _ in 0..20 {
            let x = rng.gen_range(0.0..self.world.width); let y = rng.gen_range(0.0..self.world.height);
            let m_type = match rng.gen_range(0..10) { 0..=5 => MobType::Rabbit, 6..=8 => MobType::Wolf, _ => MobType::Bear };
            let mob = Mob::new(m_type, x, y); self.world.mobs.insert(mob.id, mob);
        }
    }
    fn tick_world(&mut self) {
        let mut dead_players = Vec::new();
        for (id, player) in self.world.players.iter_mut() { SurvivalSystem::tick(player, &self.config.mechanics); if player.health <= 0.0 { dead_players.push(*id); } }
        for id in dead_players { self.world.players.remove(&id); }
        let mut rng = rand::thread_rng();
        for mob in self.world.mobs.values_mut() {
            let dx = rng.gen_range(-1.0..1.0); let dy = rng.gen_range(-1.0..1.0);
            mob.x = (mob.x + dx).clamp(0.0, self.world.width); mob.y = (mob.y + dy).clamp(0.0, self.world.height);
        }
    }
}

impl Actor for GameEngine {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) { self.spawn_initial_entities(); ctx.run_interval(std::time::Duration::from_millis(1000 / self.config.server.tick_rate), |act, _| { act.tick_world(); act.broadcast(); }); }
}

impl Handler<Join> for GameEngine {
    type Result = Option<(String, Uuid)>;
    fn handle(&mut self, msg: Join, _: &mut Context<Self>) -> Self::Result {
        if let Some(token) = msg.token { if let Some(player) = self.world.players.values_mut().find(|p| p.token == token) { let player_id = player.id; self.sessions.insert(player_id, msg.addr); return Some((token, player_id)); } }
        let mut rng = rand::thread_rng(); let spawn_x = rng.gen_range(0.0..self.world.width); let spawn_y = rng.gen_range(0.0..self.world.height);
        let token = Uuid::new_v4().to_string(); self.sessions.insert(msg.id, msg.addr);
        let player = Player::new(msg.id, token.clone(), "Guest".to_string(), spawn_x, spawn_y); self.world.players.insert(msg.id, player); Some((token, msg.id))
    }
}

impl Handler<Leave> for GameEngine { type Result = (); fn handle(&mut self, msg: Leave, _: &mut Context<Self>) { self.sessions.remove(&msg.id); } }
impl Handler<Craft> for GameEngine { type Result = (); fn handle(&mut self, msg: Craft, _: &mut Context<Self>) { if let Some(p) = self.world.players.get_mut(&msg.id) { CraftingSystem::craft(&mut p.inventory, msg.item); } } }
impl Handler<SelectSlot> for GameEngine {
    type Result = ();
    fn handle(&mut self, msg: SelectSlot, _: &mut Context<Self>) {
        if let Some(p) = self.world.players.get_mut(&msg.id) {
            if msg.slot < 7 { p.active_slot = msg.slot; }
        }
    }
}
impl Handler<UpdateName> for GameEngine { type Result = (); fn handle(&mut self, msg: UpdateName, _: &mut Context<Self>) { if let Some(p) = self.world.players.get_mut(&msg.id) { let mut name = msg.name.trim().to_string(); if name.is_empty() { name = "Guest".to_string(); } if name.len() > 12 { name.truncate(12); } p.username = name; } } }

impl Handler<Input> for GameEngine {
    type Result = ();
    fn handle(&mut self, msg: Input, _: &mut Context<Self>) {
        let interact_range = self.config.game.interact_range;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        if let Some(player) = self.world.players.get_mut(&msg.id) {
            let speed = 5.0;
            player.x = (player.x + msg.dx * speed).clamp(0.0, self.world.width);
            player.y = (player.y + msg.dy * speed).clamp(0.0, self.world.height);
            
            if msg.attack {
                // Cooldown check
                if now - player.last_attack_at >= self.config.mechanics.attack_cooldown {
                    player.last_attack_at = now;
                    let mut processed = false;
                    let active_slot_idx = player.active_slot;
                    
                    let mut should_clear_slot = false;
                    // 1. Try Use Held Item (A Button)
                    if let Some(item) = &mut player.inventory.slots[active_slot_idx] {
                        // Eat
                        if item.kind == ItemType::Berry || item.kind == ItemType::Meat || item.kind == ItemType::CookedMeat {
                            player.hunger = (player.hunger + self.config.mechanics.food_value).min(100.0);
                            item.amount -= 1; if item.amount == 0 { should_clear_slot = true; }
                            processed = true;
                        }
                        // Build
                        if !processed {
                            let s_type = match item.kind {
                                ItemType::WoodWall => Some(StructureType::Wall), ItemType::Door => Some(StructureType::Door),
                                ItemType::Torch => Some(StructureType::Torch), ItemType::Workbench => Some(StructureType::Workbench),
                                _ => None,
                            };
                            if let Some(st) = s_type {
                                let s = Structure::new(st, player.x, player.y, msg.id);
                                self.world.structures.insert(s.id, s);
                                item.amount -= 1; if item.amount == 0 { should_clear_slot = true; }
                                processed = true;
                            }
                        }
                    }
                    if should_clear_slot { player.inventory.slots[active_slot_idx] = None; }

                    // 2. Fallback to Attack/Gather (A Button)
                    if !processed {
                        let mut drops = Vec::new(); let mut collected = Vec::new();
                        for (id, res) in self.world.resources.iter_mut() {
                            if Math::dist(player.x, player.y, res.x, res.y) < interact_range {
                                 res.amount -= 1;
                                 match res.r_type { ResourceType::Tree => drops.push((ItemType::Wood, 1)), ResourceType::Rock => drops.push((ItemType::Stone, 1)), ResourceType::Food => drops.push((ItemType::Berry, 1)) }
                                 if res.amount <= 0 { collected.push(*id); }
                                 processed = true; break;
                            }
                        }
                        if !processed {
                            for (id, mob) in self.world.mobs.iter_mut() {
                                 if Math::dist(player.x, player.y, mob.x, mob.y) < interact_range {
                                     mob.health -= 5.0; if mob.health <= 0.0 { collected.push(*id); drops.push((ItemType::Meat, 1)); }
                                     processed = true; break;
                                 }
                            }
                        }
                        for (kind, amt) in drops { player.inventory.add(kind, amt); }
                        for id in collected { self.world.resources.remove(&id); self.world.mobs.remove(&id); }
                    }
                }
            }
            if msg.interact {
                // Cooldown check
                if now - player.last_interact_at >= self.config.mechanics.interact_cooldown {
                    player.last_interact_at = now;
                    /* B Button Interactions (Open Door etc) */
                }
            }
        }
    }
}

struct Math;
impl Math { fn dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 { ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt() } }
