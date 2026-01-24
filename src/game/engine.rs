use actix::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;
use crate::game::state::World;
use crate::game::entities::player::Player;
use crate::game::entities::resource::{Resource, ResourceType};
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
}

impl Actor for GameEngine {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Spawn resources
        let mut rng = rand::thread_rng();
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

        // Start ticking 20 TPS
        ctx.run_interval(std::time::Duration::from_millis(50), |act, _| {
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
                         // Only hit one at a time per tick? Or area?
                         // Let's break after one hit for simplicity/balance
                         break; 
                    }
                }
                
                player.inventory.wood += inventory_update.0;
                player.inventory.stone += inventory_update.1;
                player.inventory.food += inventory_update.2;

                for id in collected {
                    self.world.resources.remove(&id);
                }
            }
        }
    }
}
