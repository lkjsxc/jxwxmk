use actix_web::{middleware::DefaultHeaders, web, App, HttpServer};

use crate::config::Config;
use crate::game::engine::GameEngine;
use crate::server::database::Db;
use crate::server::rate_limit::RateLimiter;

pub mod database;
pub mod http;
pub mod rate_limit;
pub mod session;
pub mod static_assets;
pub mod ws;

#[derive(Clone)]
pub struct AppState {
    pub engine: actix::Addr<GameEngine>,
    pub db: Db,
    pub config: Config,
    pub session_limiter: RateLimiter,
}

pub async fn run() -> std::io::Result<()> {
    let config_dir = std::env::var("CONFIG_DIR").unwrap_or_else(|_| "config".to_string());
    let config = Config::load_from_dir(&config_dir)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@127.0.0.1:5432/jxwxmk".to_string());
    let db = database::init_pool(&database_url)
        .await
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;

    let engine = GameEngine::new(config.clone(), db.clone()).start();
    let limiter = RateLimiter::new(
        config.server.session_claims_per_minute,
        std::time::Duration::from_secs(60),
    );
    let state = AppState {
        engine,
        db,
        config: config.clone(),
        session_limiter: limiter,
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(
                DefaultHeaders::new()
                    .add((
                        "Content-Security-Policy",
                        "default-src 'self'; style-src 'self' 'unsafe-inline'; script-src 'self' 'unsafe-eval'",
                    ))
                    .add(("X-Content-Type-Options", "nosniff"))
                    .add(("X-Frame-Options", "DENY")),
            )
            .route("/health", web::get().to(http::health))
            .route("/session/claim", web::post().to(http::session_claim))
            .route("/ws", web::get().to(ws::ws_route))
            .route("/", web::get().to(http::static_index))
            .route("/{filename:.*}", web::get().to(http::static_asset))
    })
    .workers(1)
    .bind((config.server.http_addr.as_str(), config.server.http_port))?
    .run()
    .await
}
