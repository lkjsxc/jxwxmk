use std::sync::{Arc, Mutex};
use std::time::Duration;

use actix::Actor;
use actix_test::start;
use awc::Client;

use jxwxmk::config::{
    AchievementsConfig, BalanceConfig, BiomesConfig, Config, CraftingConfig, EconomyConfig,
    QuestsConfig, ServerConfig, SettlementsConfig, SpawningConfig, SurvivalConfig, WorldConfig,
};
use jxwxmk::game::GameEngine;
use jxwxmk::net::{build_app, AppState, RateLimiter};
use jxwxmk::persistence::{init_db, init_pool};

fn base_config() -> Config {
    Config::new(
        ServerConfig::default(),
        WorldConfig::default(),
        BalanceConfig::default(),
        SurvivalConfig::default(),
        CraftingConfig::default(),
        SpawningConfig::default(),
        BiomesConfig::default(),
        SettlementsConfig::default(),
        EconomyConfig::default(),
        QuestsConfig::default(),
        AchievementsConfig::default(),
    )
}

#[actix_web::test]
async fn websocket_handshake_succeeds() {
    let database_url = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => return,
    };

    let config = base_config();
    let pool = init_pool(&database_url).await.expect("db");
    init_db(&pool).await.expect("init db");

    let engine = GameEngine::new(config.clone(), pool.clone()).start();
    let limiter = Arc::new(Mutex::new(RateLimiter::new(
        config.server.session_claims_per_minute,
        Duration::from_secs(60),
    )));

    let state = AppState {
        config: config.clone(),
        engine,
        db: pool.clone(),
        claim_limiter: limiter,
    };

    let srv = start(move || build_app(state.clone()));
    let (_resp, _conn) = Client::new().ws(srv.url("/ws")).connect().await.unwrap();
}
