use std::collections::{HashMap, VecDeque};

use actix::{Actor, AsyncContext};
use sqlx::PgPool;

use crate::config::Config;
use crate::protocol::ServerMessage;

use crate::game::engine_tick::run_tick;
use crate::game::events::EngineEvent;
use crate::game::world::World;
use crate::game::engine_world;
use crate::game::entities::PlayerId;

pub struct GameEngine {
    pub config: Config,
    pub world: World,
    pub db: PgPool,
    pub input_queue: VecDeque<EngineEvent>,
    pub sessions: HashMap<PlayerId, actix::Recipient<ServerMessage>>,
    pub tick: u64,
    pub save_interval_ticks: u64,
}

impl GameEngine {
    pub fn new(config: Config, db: PgPool) -> Self {
        let mut world = World::new(config.world.seed);
        engine_world::initialize_settlements(&mut world, &config);
        let save_interval_ticks = (config.server.tick_rate * 10.0).ceil() as u64;
        Self {
            config,
            world,
            db,
            input_queue: VecDeque::new(),
            sessions: HashMap::new(),
            tick: 0,
            save_interval_ticks,
        }
    }
}

impl Actor for GameEngine {
    type Context = actix::Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let tick_rate = self.config.server.tick_rate.max(1.0);
        let interval = std::time::Duration::from_secs_f32(1.0 / tick_rate);
        ctx.run_interval(interval, |act, _ctx| {
            run_tick(act);
        });
    }
}
