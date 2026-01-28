use env_logger;
use log::{info, error};
use net::start_server;
use config::load_config;
use game::GameEngine;
use persistence::PersistenceManager;
use actix::prelude::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    info!("Starting server...");

    // 1. Load Config
    let config = match load_config("config") {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to load config: {:?}", e);
            std::process::exit(1);
        }
    };
    
    // 2. Init Persistence
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@127.0.0.1:5432/jxwxmk".to_string());
    
    let persistence = match PersistenceManager::new(&database_url).await {
        Ok(pm) => pm,
        Err(e) => {
            error!("Failed to connect to database: {:?}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = persistence.run_migrations().await {
        error!("Failed to run migrations: {:?}", e);
        std::process::exit(1);
    }
    info!("Database connected and migrations applied.");

    // 3. Start Game Engine Actor
    let game_config = config.clone();
    let game_persistence = persistence.clone();
    let game_addr = GameEngine::new(game_config, Some(game_persistence)).start();

    info!("Config loaded. Bind address: {}", config.server.bind_http);

    // 4. Start Server
    start_server(&config.server.bind_http, game_addr, persistence).await
}
