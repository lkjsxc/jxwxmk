use actix_web::{test, web, App};
use awc::Client;
use futures_util::StreamExt;

use jxwxmk::config::Config;
use jxwxmk::game::GameEngine;
use jxwxmk::server::{self, database, rate_limit::RateLimiter};

#[actix_rt::test]
async fn websocket_handshake_returns_welcome() {
    let config = Config::default();
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@127.0.0.1:5432/jxwxmk".to_string());
    let db = database::init_pool(&db_url).await.unwrap();
    let engine = GameEngine::new(config.clone(), db.clone()).start();
    let limiter = RateLimiter::new(10, std::time::Duration::from_secs(60));

    let state = server::AppState {
        engine,
        db,
        config: config.clone(),
        session_limiter: limiter,
    };

    let srv = test::start(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/ws", web::get().to(server::ws::ws_route))
    });

    let (resp, mut framed) = Client::new().ws(srv.url("/ws")).connect().await.unwrap();
    assert!(resp.status().is_success());

    if let Some(Ok(frame)) = framed.next().await {
        match frame {
            awc::ws::Frame::Text(text) => {
                let value: serde_json::Value = serde_json::from_slice(&text).unwrap();
                assert_eq!(value["type"], "welcome");
                assert!(value["token"].as_str().is_some());
            }
            _ => panic!("expected welcome text frame"),
        }
    } else {
        panic!("missing welcome message");
    }
}
