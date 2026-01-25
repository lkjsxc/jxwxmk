use actix::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;
use crate::game::state::World;
use crate::game::entities::player::Player;
use crate::game::entities::resource::{Resource, ResourceType};
use crate::game::entities::mob::{Mob, MobType};
use crate::game::entities::item::{Item, ItemType};
use crate::game::entities::structure::{Structure, StructureType};
use crate::game::systems::survival::SurvivalSystem;
use crate::game::systems::crafting::CraftingSystem;
use serde::Serialize;
use rand::Rng;

// Messages
#[derive(Message)]
#[rtype(result = "()")]
pub struct Tick;

#[derive(Message)]
#[rtype(result = "Option<(String, Uuid)>")] 
pub struct Join {
    pub id: Uuid,
    pub token: Option<String>,
    pub addr: Recipient<WorldUpdate>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Leave {
    pub id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Input {
    pub id: Uuid,
    pub dx: f64,
    pub dy: f64,
    pub attack: bool,
    pub interact: bool,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Craft {
    pub id: Uuid,
    pub item: ItemType,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SelectSlot {
    pub id: Uuid,
    pub slot: usize,
}

#[derive(Message, Clone, Serialize)]
#[rtype(result = "()")]
pub struct WorldUpdate(pub World);

pub struct GameEngine {
    world: World,
    sessions: HashMap<Uuid, Recipient<WorldUpdate>>,
}

impl GameEngine {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            sessions: HashMap::new(),
        }
    }

    fn broadcast(&self) {
        let update = WorldUpdate(self.world.clone());
        for addr in self.sessions.values() {
            addr.do_send(update.clone());
        }
    }

    fn spawn_initial_entities(&mut self) {
        let mut rng = rand::thread_rng();
        
        // Resources
        for _ in 0..100 {
            let x = rng.gen_range(0.0..self.world.width);
            let y = rng.gen_range(0.0..self.world.height);
            let r_type = match rng.gen_range(0..10) {
                0..=4 => ResourceType::Tree,
                5..=8 => ResourceType::Rock,
                _ => ResourceType::Food,
            };
            let res = Resource::new(r_type, x, y);
            self.world.resources.insert(res.id, res);
        }

        // Mobs
        for _ in 0..20 {
            let x = rng.gen_range(0.0..self.world.width);
            let y = rng.gen_range(0.0..self.world.height);
            let m_type = match rng.gen_range(0..10) {
                0..=5 => MobType::Rabbit,
                6..=8 => MobType::Wolf,
                _ => MobType::Bear,
            };
            let mob = Mob::new(m_type, x, y);
            self.world.mobs.insert(mob.id, mob);
        }
    }

    fn tick_world(&mut self) {
        // Players
        let mut dead_players = Vec::new();
        for (id, player) in self.world.players.iter_mut() {
            SurvivalSystem::tick(player);
            if player.health <= 0.0 {
                dead_players.push(*id);
            }
        }

        for id in dead_players {
            // Remove player entity
            // Also notify session? 
            // We can send a specific "GameOver" message wrapper or let the client handle "world update missing my ID".
            // Ideally explicit message.
            if let Some(addr) = self.sessions.get(&id) {
                // We can't send a custom message easily via Recipient<WorldUpdate> unless we change the type or wrap it.
                // Or we update WorldUpdate to include events.
                // For now, let's just remove the player. The client will see "myId" is not in "world.players" and trigger Game Over.
            }
            self.world.players.remove(&id);
            // We KEEP the session (so they can respawn/spectate?)
            // If we keep session but no player, subsequent Inputs might fail or need handling.
            // Client should show Game Over screen.
        }

        // Mobs (Simple AI)
        let mut rng = rand::thread_rng();
        for mob in self.world.mobs.values_mut() {
            // Random wander
            let dx = rng.gen_range(-1.0..1.0);
            let dy = rng.gen_range(-1.0..1.0);
            mob.x = (mob.x + dx).clamp(0.0, self.world.width);
            mob.y = (mob.y + dy).clamp(0.0, self.world.height);
        }
    }
}

impl Actor for GameEngine {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.spawn_initial_entities();

        // Start ticking 20 TPS
        ctx.run_interval(std::time::Duration::from_millis(50), |act, _| {
            act.tick_world();
            act.broadcast();
        });
    }
}

impl Handler<Join> for GameEngine {
    type Result = Option<(String, Uuid)>;
    fn handle(&mut self, msg: Join, _: &mut Context<Self>) -> Self::Result {
        // Check for reconnection
        if let Some(token) = msg.token {
            if let Some(player) = self.world.players.values_mut().find(|p| p.token == token) {
                let player_id = player.id;
                self.sessions.insert(player_id, msg.addr);
                return Some((token, player_id));
            }
        }

        // New Player
        let mut rng = rand::thread_rng();
        let spawn_x = rng.gen_range(0.0..self.world.width);
        let spawn_y = rng.gen_range(0.0..self.world.height);

        let token = Uuid::new_v4().to_string();
        self.sessions.insert(msg.id, msg.addr);
        let player = Player::new(msg.id, token.clone(), "Guest".to_string(), spawn_x, spawn_y);
        self.world.players.insert(msg.id, player);
        Some((token, msg.id))
    }
}

impl Handler<Leave> for GameEngine {
    type Result = ();
    fn handle(&mut self, msg: Leave, _: &mut Context<Self>) {
        // If we treat msg.id as SessionID, we need to know which PlayerID it maps to.
        // But currently assumed SessionID == PlayerID.
        // If we reconnect, we might have a mismatch if we didn't update the WS actor's ID.
        // For simplicity: We will update WS actor's ID to match PlayerID on reconnect?
        // Or we just remove from sessions.
        
        self.sessions.remove(&msg.id);
        // Persistence: Do NOT remove player
        // self.world.players.remove(&msg.id); 
    }
}

impl Handler<Craft> for GameEngine {
    type Result = ();
    fn handle(&mut self, msg: Craft, _: &mut Context<Self>) {
        if let Some(player) = self.world.players.get_mut(&msg.id) {
            CraftingSystem::craft(&mut player.inventory, msg.item);
        }
    }
}

impl Handler<SelectSlot> for GameEngine {
    type Result = ();
    fn handle(&mut self, msg: SelectSlot, _: &mut Context<Self>) {
        if let Some(player) = self.world.players.get_mut(&msg.id) {
            // Validate slot (0-9 for hotbar, or allow full inventory selection?)
            // Usually only hotbar is "active". 
            if msg.slot < 10 {
                player.active_slot = msg.slot;
            }
        }
    }
}

impl Handler<Input> for GameEngine {
    type Result = ();
    fn handle(&mut self, msg: Input, _: &mut Context<Self>) {
        if let Some(player) = self.world.players.get_mut(&msg.id) {
            let speed = 5.0;
            player.x += msg.dx * speed;
            player.y += msg.dy * speed;

            // Clamp
            player.x = player.x.max(0.0).min(self.world.width);
            player.y = player.y.max(0.0).min(self.world.height);
            
            // Gather/Attack
            if msg.attack {
                let px = player.x;
                let py = player.y;
                let mut collected = Vec::new();
                let mut drops = Vec::new();
                let mut mobs_hit = Vec::new();

                // Check Resources
                for (id, res) in self.world.resources.iter_mut() {
                    let dist = ((px - res.x).powi(2) + (py - res.y).powi(2)).sqrt();
                    if dist < 40.0 {
                         res.amount -= 1;
                         match res.r_type {
                             ResourceType::Tree => drops.push((ItemType::Wood, 1)),
                             ResourceType::Rock => drops.push((ItemType::Stone, 1)),
                             ResourceType::Food => drops.push((ItemType::Berry, 1)),
                         }
                         if res.amount <= 0 {
                             collected.push(*id);
                         }
                         break; // Hit one per tick
                    }
                }

                // Check Mobs
                for (id, mob) in self.world.mobs.iter_mut() {
                     let dist = ((px - mob.x).powi(2) + (py - mob.y).powi(2)).sqrt();
                     if dist < 40.0 {
                         mob.health -= 5.0; 
                         if mob.health <= 0.0 {
                             mobs_hit.push(*id);
                             drops.push((ItemType::Meat, 1));
                         }
                         break;
                     }
                }
                
                // Add to inventory
                for (kind, amt) in drops {
                    player.inventory.add(kind, amt);
                }

                for id in collected {
                    self.world.resources.remove(&id);
                }
                for id in mobs_hit {
                    self.world.mobs.remove(&id);
                }
            }
            
            // Interact / Build
            if msg.interact {
                // Check if holding a buildable item
                // For now, assume Slot 0 is hotbar active slot (need to implement slot selection logic)
                // Actually, let's just use the first valid buildable item in hotbar for MVP or check active_slot
                // `active_slot` is in Player.
                
                let active_slot_idx = player.active_slot;
                if active_slot_idx < player.inventory.slots.len() {
                    let should_build = if let Some(item) = &mut player.inventory.slots[active_slot_idx] {
                         let s_type = match item.kind {
                             ItemType::WoodWall => Some(StructureType::Wall),
                             ItemType::Door => Some(StructureType::Door),
                             ItemType::Torch => Some(StructureType::Torch),
                             ItemType::Workbench => Some(StructureType::Workbench),
                             _ => None,
                         };

                         if let Some(s_type) = s_type {
                             // Build it
                             item.amount -= 1;
                             if item.amount == 0 {
                                 player.inventory.slots[active_slot_idx] = None;
                             }
                             Some(s_type)
                         } else {
                             None
                         }
                    } else {
                        None
                    };

                    if let Some(s_type) = should_build {
                         let s = Structure::new(s_type, player.x, player.y, msg.id);
                         self.world.structures.insert(s.id, s);
                    }
                }
            }
        }
    }
}