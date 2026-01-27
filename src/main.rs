mod config;
mod game;
mod server;

use crate::config::AppConfig;
use env_logger;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Load config
    AppConfig::load();
    let config = AppConfig::get();
    log::info!("Config loaded. Tick rate: {}", config.server.tick_rate);

    // Database
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@127.0.0.1:5432/kkmypk".to_string());
    let db_pool = server::database::init_pool(&db_url).await.expect("Failed to connect to database");
    log::info!("Database connected");

    // Start game engine
    let game_addr = game::start(config.server.tick_rate, db_pool);

    // Start server
    server::start(config.server.port, game_addr).await
}