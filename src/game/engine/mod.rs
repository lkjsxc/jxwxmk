pub mod messages;
pub mod handlers;
pub mod logic;

pub use messages::*;

use actix::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;
use crate::game::state::World;
use crate::game::config::AppConfig;

pub struct GameEngine { 
    pub(super) world: World, 
    pub(super) sessions: HashMap<Uuid, Recipient<ServerMessage>>, 
    pub(super) config: AppConfig 
}

impl GameEngine {
    pub fn new() -> Self { 
        let config = AppConfig::load();
        Self { 
            world: World::new(config.game.world_width, config.game.world_height), 
            sessions: HashMap::new(), 
            config 
        } 
    }
}

impl Actor for GameEngine {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) { 
        self.spawn_initial_entities(); 
        ctx.run_interval(std::time::Duration::from_millis(1000 / self.config.server.tick_rate), |act, _| { 
            act.tick_world(); 
            act.broadcast(); 
        }); 
    }
}
