mod broadcast;
mod handlers_input;
mod handlers_session;
mod helpers;
mod messages;
mod streaming;
mod tick;

use std::collections::{HashMap, VecDeque};
use std::time::Duration;

use actix::prelude::*;
use uuid::Uuid;

use crate::config::Config;
use crate::game::world::World;
use crate::protocol::server::ServerMessage;
use crate::server::database::Db;

pub use messages::{
    AcceptQuestMsg, CraftMsg, InputMsg, Join, JoinResult, Leave, NameMsg, NpcActionMsg,
    RevokePlayer, SlotMsg, SpawnMsg, SwapSlotsMsg, TradeMsg,
};

pub struct GameEngine {
    pub(crate) config: Config,
    pub(crate) world: World,
    pub(crate) db: Db,
    pub(crate) sessions: HashMap<Uuid, SessionHandle>,
    pub(crate) player_sessions: HashMap<Uuid, Uuid>,
    pub(crate) queue: VecDeque<tick::EngineEvent>,
    pub(crate) tick_counter: u64,
}

pub(crate) struct SessionHandle {
    pub player_id: Uuid,
    pub addr: Recipient<ServerMessage>,
}

impl GameEngine {
    pub fn new(config: Config, db: Db) -> Self {
        let mut world = World::new(config.world.seed);
        world.spawn_settlement(&config);
        Self {
            config,
            world,
            db,
            sessions: HashMap::new(),
            player_sessions: HashMap::new(),
            queue: VecDeque::new(),
            tick_counter: 0,
        }
    }

    pub(crate) fn enqueue(&mut self, event: tick::EngineEvent) {
        if self.queue.len() >= self.config.server.input_queue_limit {
            return;
        }
        self.queue.push_back(event);
    }
}

impl Actor for GameEngine {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let tick_rate = self.config.server.tick_rate;
        ctx.run_interval(Duration::from_secs_f32(1.0 / tick_rate), |actor, _ctx| {
            actor.tick();
        });
    }
}
