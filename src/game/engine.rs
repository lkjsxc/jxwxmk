use actix::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;
use crate::game::state::World;
use crate::game::entities::player::Player;
use crate::game::entities::resource::{Resource, ResourceType};
use crate::game::entities::mob::{Mob, MobType};
use crate::game::systems::survival::SurvivalSystem;
use serde::Serialize;
use rand::Rng;

// Messages
#[derive(Message)]
#[rtype(result = "()")]
pub struct Tick;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    pub id: Uuid,
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
        for player in self.world.players.values_mut() {
            SurvivalSystem::tick(player);
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
    type Result = ();
    fn handle(&mut self, msg: Join, _: &mut Context<Self>) {
        self.sessions.insert(msg.id, msg.addr);
        let player = Player::new(msg.id, "Guest".to_string(), 100.0, 100.0);
        self.world.players.insert(msg.id, player);
    }
}

impl Handler<Leave> for GameEngine {
    type Result = ();
    fn handle(&mut self, msg: Leave, _: &mut Context<Self>) {
        self.sessions.remove(&msg.id);
        self.world.players.remove(&msg.id);
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
                let mut inventory_update = (0, 0, 0); // wood, stone, food
                let mut mobs_hit = Vec::new();

                // Check Resources
                for (id, res) in self.world.resources.iter_mut() {
                    let dist = ((px - res.x).powi(2) + (py - res.y).powi(2)).sqrt();
                    if dist < 40.0 {
                         res.amount -= 1;
                         match res.r_type {
                             ResourceType::Tree => inventory_update.0 += 1,
                             ResourceType::Rock => inventory_update.1 += 1,
                             ResourceType::Food => inventory_update.2 += 1,
                         }
                         if res.amount <= 0 {
                             collected.push(*id);
                         }
                         break; // Hit one per tick
                    }
                }

                // Check Mobs (Simple Hit)
                for (id, mob) in self.world.mobs.iter_mut() {
                     let dist = ((px - mob.x).powi(2) + (py - mob.y).powi(2)).sqrt();
                     if dist < 40.0 {
                         mob.health -= 5.0; // Basic punch damage
                         if mob.health <= 0.0 {
                             mobs_hit.push(*id);
                             inventory_update.2 += 2; // Meat
                         }
                         break;
                     }
                }
                
                player.inventory.wood += inventory_update.0;
                player.inventory.stone += inventory_update.1;
                player.inventory.food += inventory_update.2;

                for id in collected {
                    self.world.resources.remove(&id);
                }
                for id in mobs_hit {
                    self.world.mobs.remove(&id);
                }
            }
        }
    }
}