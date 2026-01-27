use std::env;
use std::path::PathBuf;

use actix::Actor;
use actix_web::HttpServer;

use jxwxmk::config::load_config;
use jxwxmk::game::GameEngine;
use jxwxmk::net::{build_app, AppState, RateLimiter};
use jxwxmk::persistence::{init_db, init_pool};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let config_dir = env::var("CONFIG_DIR").unwrap_or_else(|_| "./config".to_string());
    let config_path = PathBuf::from(config_dir);
    let config = load_config(&config_path);

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@127.0.0.1:5432/jxwxmk".to_string());
    let pool = init_pool(&database_url)
        .await
        .expect("Failed to connect to database");
    init_db(&pool).await.expect("Failed to init database");

    let engine = GameEngine::new(config.clone(), pool.clone()).start();

    let limiter = std::sync::Arc::new(std::sync::Mutex::new(
        RateLimiter::new(
            config.server.session_claims_per_minute,
            std::time::Duration::from_secs(60),
        ),
    ));

    let state = AppState {
        config: config.clone(),
        engine,
        db: pool.clone(),
        claim_limiter: limiter,
    };

    let bind_addr = format!("{}:{}", config.server.http_addr, config.server.http_port);
    HttpServer::new(move || build_app(state.clone()))
        .workers(1)
        .bind(bind_addr)?
        .run()
        .await
}
