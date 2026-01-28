mod protocol;
mod persistence;
mod game;
mod net;
mod config;

use actix_web::{App, HttpServer, middleware, web};
use actix::Actor;
use std::env;
use std::path::Path;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Load env vars if .env exists (optional)
    dotenvy::dotenv().ok();

    // Load Config
    let config_dir = env::var("CONFIG_DIR").unwrap_or_else(|_| "config".to_string());
    let game_config = config::loader::load_config(Path::new(&config_dir));

    let port = env::var("PORT").unwrap_or_else(|_| game_config.server.port.to_string());
    let bind_addr = format!("0.0.0.0:{}", port);
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    log::info!("Starting server on {}", bind_addr);
    
    // DB Init
    let pool = persistence::init_pool(&database_url).await.expect("Failed to init DB pool");
    persistence::run_migrations(&pool).await.expect("Failed to run migrations");

    // Start Game Engine
    let game_engine = game::GameEngine::new(game_config).start();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(game_engine.clone()))
            .route("/health", web::get().to(|| async { "OK" }))
            .route("/session/claim", web::post().to(net::http::claim_session))
            .route("/ws", web::get().to(net::ws::ws_route))
            .route("/{filename:.*}", web::get().to(net::http::serve_asset))
            .route("/", web::get().to(net::http::serve_index))
    })
    .bind(bind_addr)?
    .run()
    .await
}
