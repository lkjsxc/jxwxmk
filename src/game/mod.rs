pub mod engine;
pub mod world_state;
pub mod systems_survival;
pub mod systems_interaction;
pub mod spawning_and_ai;
pub mod systems_crafting;
pub mod systems_ai;
pub mod systems_barriers;
pub mod quests;
pub mod achievements;

use actix::prelude::*;
use engine::GameEngine;
use crate::server::database::DbPool;

pub fn start(tick_rate: u64, db_pool: DbPool) -> Addr<GameEngine> {
    GameEngine::new(tick_rate, db_pool).start()
}
