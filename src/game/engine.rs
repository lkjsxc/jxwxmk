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

#[derive(Message)] #[rtype(result = "()")] pub struct Tick;
#[derive(Message)] #[rtype(result = "Option<(String, Uuid)>")] pub struct Join { pub id: Uuid, pub token: Option<String>, pub addr: Recipient<WorldUpdate> }
#[derive(Message)] #[rtype(result = "()")] pub struct Leave { pub id: Uuid }
#[derive(Message)] #[rtype(result = "()")] pub struct Input { pub id: Uuid, pub dx: f64, pub dy: f64, pub attack: bool, pub interact: bool }
#[derive(Message)] #[rtype(result = "()")] pub struct Craft { pub id: Uuid, pub item: ItemType }
#[derive(Message)] #[rtype(result = "()")] pub struct SelectSlot { pub id: Uuid, pub slot: usize }
#[derive(Message)] #[rtype(result = "()")] pub struct UpdateName { pub id: Uuid, pub name: String }
#[derive(Message)] #[rtype(result = "()")] pub struct SwapSlots { pub id: Uuid, pub from: usize, pub to: usize }
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
        let mut dead_p = Vec::new();
        for (id, p) in self.world.players.iter_mut() { SurvivalSystem::tick(p, &self.config.mechanics); if p.health <= 0.0 { dead_p.push(*id); } }
        for id in dead_p { self.world.players.remove(&id); }

        let mut rng = rand::thread_rng();
        let player_ids: Vec<Uuid> = self.world.players.keys().cloned().collect();
        for mob in self.world.mobs.values_mut() {
            if mob.m_type == MobType::Rabbit {
                let dx = rng.gen_range(-1.0..1.0); let dy = rng.gen_range(-1.0..1.0);
                mob.x = (mob.x + dx).clamp(0.0, self.world.width); mob.y = (mob.y + dy).clamp(0.0, self.world.height);
            } else {
                // Hostile logic
                let mut target = None; let mut min_dist = 300.0;
                for pid in &player_ids {
                    let p = &self.world.players[pid];
                    let d = Math::dist(mob.x, mob.y, p.x, p.y);
                    if d < min_dist { min_dist = d; target = Some(p); }
                }
                if let Some(t) = target {
                    let dx = t.x - mob.x; let dy = t.y - mob.y;
                    if min_dist > 20.0 { mob.x += dx / min_dist * 2.0; mob.y += dy / min_dist * 2.0; }
                    else {
                        // Attack Player
                        // In a real system we'd track mob attack cooldowns.
                        // For MVP, we'll let the player's survival system handle it or apply here.
                    }
                }
            }
        }
        // Mob damage pass
        let mut dmg_to_apply = Vec::new();
        for mob in self.world.mobs.values() {
            if mob.m_type == MobType::Rabbit { continue; }
            for (pid, p) in self.world.players.iter() {
                if Math::dist(mob.x, mob.y, p.x, p.y) < 30.0 {
                    dmg_to_apply.push((*pid, match mob.m_type { MobType::Wolf => 0.5, MobType::Bear => 1.5, _ => 0.0 }));
                }
            }
        }
        for (pid, d) in dmg_to_apply { if let Some(p) = self.world.players.get_mut(&pid) { p.health -= d; } }
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
impl Handler<SelectSlot> for GameEngine { type Result = (); fn handle(&mut self, msg: SelectSlot, _: &mut Context<Self>) { if let Some(p) = self.world.players.get_mut(&msg.id) { if msg.slot < 7 { p.active_slot = msg.slot; } } } }
impl Handler<UpdateName> for GameEngine { type Result = (); fn handle(&mut self, msg: UpdateName, _: &mut Context<Self>) { if let Some(p) = self.world.players.get_mut(&msg.id) { let mut n = msg.name.trim().to_string(); if n.is_empty() { n = "Guest".to_string(); } if n.len() > 12 { n.truncate(12); } p.username = n; } } }
impl Handler<SwapSlots> for GameEngine { type Result = (); fn handle(&mut self, msg: SwapSlots, _: &mut Context<Self>) { if let Some(p) = self.world.players.get_mut(&msg.id) { if msg.from < p.inventory.slots.len() && msg.to < p.inventory.slots.len() { p.inventory.slots.swap(msg.from, msg.to); } } } }

impl Handler<Input> for GameEngine {
    type Result = ();
    fn handle(&mut self, msg: Input, _: &mut Context<Self>) {
        let range = self.config.game.interact_range;
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64;
        let (px, py, slot) = if let Some(p) = self.world.players.get(&msg.id) { (p.x, p.y, p.active_slot) } else { return; };
        
        if let Some(player) = self.world.players.get_mut(&msg.id) {
            player.x = (player.x + msg.dx * 5.0).clamp(0.0, self.world.width); player.y = (player.y + msg.dy * 5.0).clamp(0.0, self.world.height);
            if msg.attack && now - player.last_attack_at >= self.config.mechanics.attack_cooldown {
                player.last_attack_at = now; let mut proc = false; let mut clear = false;
                
                let mut tool_dmg = 2.0;
                let mut rock_mult = 1.0;
                if let Some(item) = &player.inventory.slots[slot] {
                    if item.kind == ItemType::WoodPickaxe { tool_dmg = 4.0; rock_mult = 2.0; }
                    if item.kind == ItemType::StonePickaxe { tool_dmg = 8.0; rock_mult = 3.0; }
                }

                if let Some(item) = &mut player.inventory.slots[slot] {
                    if item.kind == ItemType::Berry || item.kind == ItemType::Meat || item.kind == ItemType::CookedMeat {
                        player.hunger = (player.hunger + self.config.mechanics.food_value).min(100.0); item.amount -= 1; if item.amount == 0 { clear = true; } proc = true;
                    }
                    if !proc {
                        let st = match item.kind { ItemType::WoodWall => Some(StructureType::Wall), ItemType::Door => Some(StructureType::Door), ItemType::Torch => Some(StructureType::Torch), ItemType::Workbench => Some(StructureType::Workbench), _ => None };
                        if let Some(t) = st { let s = Structure::new(t, player.x, player.y, msg.id); self.world.structures.insert(s.id, s); item.amount -= 1; if item.amount == 0 { clear = true; } proc = true; }
                    }
                }
                if clear { player.inventory.slots[slot] = None; }
                if !proc {
                    // Try hit resources
                    let mut res_id = None;
                    for (id, r) in self.world.resources.iter() { if Math::dist(px, py, r.x, r.y) < range { res_id = Some(*id); break; } }
                    if let Some(rid) = res_id {
                        let r = self.world.resources.get_mut(&rid).unwrap();
                        r.amount -= (tool_dmg * if r.r_type == ResourceType::Rock { rock_mult } else { 1.0 }) as i32;
                        if r.amount <= 0 {
                            let drop = match r.r_type { ResourceType::Tree => (ItemType::Wood, 5), ResourceType::Rock => (ItemType::Stone, 3), ResourceType::Food => (ItemType::Berry, 2) };
                            player.inventory.add(drop.0, drop.1); self.world.resources.remove(&rid);
                        }
                        proc = true;
                    }
                }
                if !proc {
                    // Try hit mobs
                    let mut mid = None;
                    for (id, m) in self.world.mobs.iter() { if Math::dist(px, py, m.x, m.y) < range { mid = Some(*id); break; } }
                    if let Some(id) = mid {
                        let m = self.world.mobs.get_mut(&id).unwrap(); m.health -= tool_dmg;
                        if m.health <= 0.0 { player.inventory.add(ItemType::Meat, 2); self.world.mobs.remove(&id); }
                        proc = true;
                    }
                }
                if !proc {
                    // Try hit other players
                    let mut tid = None;
                    for (id, p) in self.world.players.iter() { if *id != msg.id && Math::dist(px, py, p.x, p.y) < range { tid = Some(*id); break; } }
                    if let Some(id) = tid { let p = self.world.players.get_mut(&id).unwrap(); p.health -= tool_dmg; }
                }
            }
        }
    }
}

struct Math;
impl Math { fn dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 { ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt() } }
