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
use crate::game::systems::achievements::{AchievementSystem, Achievement};
use crate::game::config::AppConfig;
use serde::Serialize;
use rand::Rng;

#[derive(Message, Clone, Serialize)] #[rtype(result = "()")] 
pub enum ServerMessage {
    WorldUpdate(World),
    AchievementUnlocked(Achievement),
}

#[derive(Message)] #[rtype(result = "()")] pub struct Tick;
#[derive(Message)] #[rtype(result = "Option<(String, Uuid)>")] pub struct Join { pub id: Uuid, pub token: Option<String>, pub addr: Recipient<ServerMessage> }
#[derive(Message)] #[rtype(result = "()")] pub struct Leave { pub id: Uuid }
#[derive(Message)] #[rtype(result = "()")] pub struct Input { pub id: Uuid, pub dx: f64, pub dy: f64, pub attack: bool, pub interact: bool }
#[derive(Message)] #[rtype(result = "()")] pub struct Craft { pub id: Uuid, pub item: ItemType }
#[derive(Message)] #[rtype(result = "()")] pub struct SelectSlot { pub id: Uuid, pub slot: usize }
#[derive(Message)] #[rtype(result = "()")] pub struct UpdateName { pub id: Uuid, pub name: String }
#[derive(Message)] #[rtype(result = "()")] pub struct SwapSlots { pub id: Uuid, pub from: usize, pub to: usize }

pub struct GameEngine { world: World, sessions: HashMap<Uuid, Recipient<ServerMessage>>, config: AppConfig }

impl GameEngine {
    pub fn new() -> Self { Self { world: World::new(), sessions: HashMap::new(), config: AppConfig::load() } }
    fn broadcast(&self) { let msg = ServerMessage::WorldUpdate(self.world.clone()); for addr in self.sessions.values() { addr.do_send(msg.clone()); } }
    
    fn check_achievements(&mut self, player_id: Uuid) {
        if let Some(player) = self.world.players.get_mut(&player_id) {
            let unlocked = AchievementSystem::check(player);
            if !unlocked.is_empty() {
                if let Some(addr) = self.sessions.get(&player_id) {
                    for ach in unlocked { addr.do_send(ServerMessage::AchievementUnlocked(ach)); }
                }
            }
        }
    }

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
        let mut dead_p = Vec::new();
        for (id, p) in self.world.players.iter_mut() { SurvivalSystem::tick(p, &self.config.mechanics); if p.health <= 0.0 { dead_p.push(*id); } }
        for id in dead_p { 
            if let Some(p) = self.world.players.get_mut(&id) { p.stats.deaths += 1; } // Keep stats? No, player removed. Need persistence.
            self.world.players.remove(&id); 
        }

        let mut rng = rand::thread_rng();
        let player_ids: Vec<Uuid> = self.world.players.keys().cloned().collect();
        for mob in self.world.mobs.values_mut() {
            if mob.m_type == MobType::Rabbit {
                let dx = rng.gen_range(-1.0..1.0); let dy = rng.gen_range(-1.0..1.0);
                mob.x = (mob.x + dx).clamp(0.0, self.world.width); mob.y = (mob.y + dy).clamp(0.0, self.world.height);
            } else {
                let mut target = None; let mut min_dist = 300.0;
                for pid in &player_ids {
                    let p = &self.world.players[pid];
                    let d = Math::dist(mob.x, mob.y, p.x, p.y);
                    if d < min_dist { min_dist = d; target = Some(p); }
                }
                if let Some(t) = target {
                    let dx = t.x - mob.x; let dy = t.y - mob.y;
                    if min_dist > 20.0 { mob.x += dx / min_dist * 2.0; mob.y += dy / min_dist * 2.0; }
                }
            }
        }
        // Mob damage
        let mut dmg_to_apply = Vec::new();
        for mob in self.world.mobs.values() {
            if mob.m_type == MobType::Rabbit { continue; }
            for (pid, p) in self.world.players.iter() {
                if Math::dist(mob.x, mob.y, p.x, p.y) < 30.0 {
                    dmg_to_apply.push((*pid, match mob.m_type { MobType::Wolf => 0.5, MobType::Bear => 1.5, _ => 0.0 }));
                }
            }
        }
        for (pid, d) in dmg_to_apply { 
            if let Some(p) = self.world.players.get_mut(&pid) { 
                p.health -= d; p.stats.damage_taken += d; 
                if p.health <= 0.0 { p.stats.deaths += 1; } // Check ach before death?
            }
            self.check_achievements(pid);
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
        let mut rng = rand::thread_rng();
        let (cx, cy, r) = (self.world.width / 2.0, self.world.height / 2.0, self.config.game.spawn_radius);
        let angle = rng.gen_range(0.0..std::f64::consts::TAU); let dist = rng.gen_range(0.0..r);
        let (sx, sy) = (cx + angle.cos() * dist, cy + angle.sin() * dist);
        let token = Uuid::new_v4().to_string(); self.sessions.insert(msg.id, msg.addr);
        let player = Player::new(msg.id, token.clone(), "Guest".to_string(), sx, sy); self.world.players.insert(msg.id, player); Some((token, msg.id))
    }
}

impl Handler<Leave> for GameEngine { type Result = (); fn handle(&mut self, msg: Leave, _: &mut Context<Self>) { self.sessions.remove(&msg.id); } }
impl Handler<Craft> for GameEngine { 
    type Result = (); 
    fn handle(&mut self, msg: Craft, _: &mut Context<Self>) { 
        if let Some(p) = self.world.players.get_mut(&msg.id) { 
            CraftingSystem::craft(&mut p.inventory, msg.item); 
            p.stats.items_crafted += 1;
        }
        self.check_achievements(msg.id);
    } 
}
impl Handler<SelectSlot> for GameEngine { type Result = (); fn handle(&mut self, msg: SelectSlot, _: &mut Context<Self>) { if let Some(p) = self.world.players.get_mut(&msg.id) { if msg.slot < 7 { p.active_slot = msg.slot; } } } }
impl Handler<UpdateName> for GameEngine { type Result = (); fn handle(&mut self, msg: UpdateName, _: &mut Context<Self>) { if let Some(p) = self.world.players.get_mut(&msg.id) { let mut n = msg.name.trim().to_string(); if n.is_empty() { n = "Guest".to_string(); } if n.len() > 12 { n.truncate(12); } p.username = n; } } }
impl Handler<SwapSlots> for GameEngine { type Result = (); fn handle(&mut self, msg: SwapSlots, _: &mut Context<Self>) { if let Some(p) = self.world.players.get_mut(&msg.id) { if msg.from < p.inventory.slots.len() && msg.to < p.inventory.slots.len() { p.inventory.slots.swap(msg.from, msg.to); } } } }

use crate::game::systems::interaction::InteractionSystem;

// ... (existing imports)

impl Handler<Input> for GameEngine {
    type Result = ();
    fn handle(&mut self, msg: Input, _: &mut Context<Self>) {
        // Movement
        InteractionSystem::handle_movement(&mut self.world, msg.id, msg.dx, msg.dy);

        // Attack / Interact
        if msg.attack {
            let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64;
            InteractionSystem::handle_attack(&mut self.world, &self.config, msg.id, now);
        }
        
        self.check_achievements(msg.id);
    }
}

struct Math;
impl Math { fn dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 { ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt() } }