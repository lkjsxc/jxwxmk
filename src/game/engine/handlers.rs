use actix::prelude::*;
use uuid::Uuid;
use rand::Rng;
use crate::game::engine::GameEngine;
use crate::game::engine::messages::*;
use crate::game::entities::player::Player;
use crate::game::systems::crafting::CraftingSystem;
use crate::game::systems::interaction::{InteractionSystem, InteractionEvent};

impl Handler<Join> for GameEngine {
    type Result = Option<(String, Uuid)>;
    fn handle(&mut self, msg: Join, _: &mut Context<Self>) -> Self::Result {
        if let Some(token) = msg.token { if let Some(player) = self.world.players.values_mut().find(|p| p.token == token) { let player_id = player.id; self.sessions.insert(player_id, msg.addr); return Some((token, player_id)); } }
        let token = Uuid::new_v4().to_string(); 
        self.sessions.insert(msg.id, msg.addr);
        let player = Player::new(msg.id, token.clone(), "Guest".to_string(), 0.0, 0.0);
        self.world.players.insert(msg.id, player); 
        Some((token, msg.id))
    }
}

impl Handler<Spawn> for GameEngine {
    type Result = ();
    fn handle(&mut self, msg: Spawn, _: &mut Context<Self>) {
        if let Some(player) = self.world.players.get_mut(&msg.id) {
            let mut rng = rand::thread_rng();
            let (cx, cy, r) = (self.world.width / 2.0, self.world.height / 2.0, self.config.game.spawn_radius);
            let angle = rng.gen_range(0.0..std::f64::consts::TAU); let dist = rng.gen_range(0.0..r);
            player.x = cx + angle.cos() * dist;
            player.y = cy + angle.sin() * dist;
            player.health = 100.0;
            player.hunger = 100.0;
            player.spawned = true;
        }
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

impl Handler<Input> for GameEngine {
    type Result = ();
    fn handle(&mut self, msg: Input, _: &mut Context<Self>) {
        if let Some(p) = self.world.players.get(&msg.id) { if !p.spawned { return; } } else { return; }
        InteractionSystem::handle_movement(&mut self.world, msg.id, msg.dx, msg.dy);
        if msg.attack {
            let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64;
            let events = InteractionSystem::handle_attack(&mut self.world, &self.config, msg.id, now);
            for event in events {
                match event {
                    InteractionEvent::LevelUp { tool, level } => {
                        if let Some(addr) = self.sessions.get(&msg.id) {
                            addr.do_send(ServerMessage::Notification { 
                                title: "Tool Level Up!".to_string(), 
                                message: format!("{} reached level {}", tool, level), 
                                color: "#4f4".to_string() 
                            });
                        }
                    }
                }
            }
        }
        self.check_achievements(msg.id);
    }
}
