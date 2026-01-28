use actix_web::{web, App, HttpServer, middleware};
use actix::Addr;
use game::GameEngine;
use persistence::PersistenceManager;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::{Instant, Duration};

pub mod routes;
pub mod ws_actor;

pub struct ServerState {
    pub game_addr: Addr<GameEngine>,
    pub persistence: PersistenceManager,
    pub rate_limiter: Arc<Mutex<RateLimiter>>,
}

pub struct RateLimiter {
    pub requests: HashMap<String, (u32, Instant)>,
    pub limit: u32,
    pub window: Duration,
}

impl RateLimiter {
    pub fn new(limit: u32, window_secs: u64) -> Self {
        Self {
            requests: HashMap::new(),
            limit,
            window: Duration::from_secs(window_secs),
        }
    }

    pub fn check(&mut self, ip: String) -> bool {
        let now = Instant::now();
        let (count, start) = self.requests.entry(ip).or_insert((0, now));
        
        if now.duration_since(*start) > self.window {
            *count = 1;
            *start = now;
            true
        } else {
            if *count < self.limit {
                *count += 1;
                true
            } else {
                false
            }
        }
    }
}

pub async fn start_server(
    bind_addr: &str,
    game_addr: Addr<GameEngine>,
    persistence: PersistenceManager,
) -> std::io::Result<()> {
    let rate_limiter = Arc::new(Mutex::new(RateLimiter::new(10, 60))); // 10 per minute

    let state = web::Data::new(ServerState {
        game_addr,
        persistence,
        rate_limiter,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(middleware::Logger::default())
            // Security Headers
            .wrap(middleware::DefaultHeaders::new()
                .add(("Content-Security-Policy", "default-src 'self'; style-src 'self' 'unsafe-inline'; script-src 'self' 'unsafe-eval'"))
                .add(("X-Content-Type-Options", "nosniff"))
                .add(("X-Frame-Options", "DENY")))
            .route("/health", web::get().to(routes::health))
            .route("/metrics", web::get().to(routes::metrics))
            .route("/session/claim", web::post().to(routes::claim_session))
            .route("/ws", web::get().to(routes::ws_index))
            .route("/", web::get().to(routes::index))
            .route("/{filename:.*}", web::get().to(routes::static_asset))
    })
    .bind(bind_addr)?
    .run()
    .await
}