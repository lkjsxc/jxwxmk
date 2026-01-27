mod http;
mod rate_limit;
mod security;
mod session;
mod session_map;
mod ws;

use std::sync::{Arc, Mutex};

use actix::Addr;
use sqlx::PgPool;

use crate::config::Config;
use crate::game::GameEngine;

pub use http::build_app;
pub use rate_limit::RateLimiter;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub engine: Addr<GameEngine>,
    pub db: PgPool,
    pub claim_limiter: Arc<Mutex<rate_limit::RateLimiter>>,
}
