use actix_web::{web, App, HttpServer, middleware, http::header};
use std::sync::Arc;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::time::interval;

mod handlers;
mod logging;
mod metrics;
mod rate_limit;

use crate::metrics::GameMetrics;

#[derive(Clone)]
pub struct RateLimiter {
    requests: Arc<RwLock<HashMap<String, Vec<Instant>>>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_secs: u64) -> Self {
        RateLimiter {
            requests: Arc::new(RwLock::new(HashMap::new())),
            max_requests,
            window: Duration::from_secs(window_secs),
        }
    }

    pub async fn check_rate(&self,
        key: &str,
    ) -> bool {
        let mut requests = self.requests.write().await;
        let now = Instant::now();
        
        let entry = requests.entry(key.to_string()).or_insert_with(Vec::new);
        
        entry.retain(|&time| now.duration_since(time) < self.window);
        
        if entry.len() >= self.max_requests {
            return false;
        }
        
        entry.push(now);
        true
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    log::info!("Starting JXWXMK server...");
    
    // Load configuration
    let config = match config::Config::load_from_dir(std::path::Path::new("/app/config")) {
        Ok(cfg) => {
            log::info!("Configuration loaded successfully");
            cfg
        }
        Err(e) => {
            log::warn!("Failed to load configuration: {}. Using defaults.", e);
            config::Config::load_defaults()
        }
    };
    
    let session_registry = Arc::new(net::SessionRegistry::new());
    let rate_limiter = RateLimiter::new(60, 60); // 60 requests per minute
    let metrics = Arc::new(GameMetrics::new());
    
    // Start game engine in background
    let game_engine = game::GameEngine::new(config);
    let world_handle = game_engine.get_world_handle();
    let metrics_clone = Arc::clone(&metrics);
    
    tokio::spawn(async move {
        game_engine.run().await;
    });
    
    // Start metrics collection task
    let metrics_for_collection = Arc::clone(&metrics);
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            
            let player_count = world_handle.get_player_count().await;
            let chunk_count = world_handle.get_active_chunk_count().await;
            
            metrics_for_collection.set_active_players(player_count);
            metrics_for_collection.set_active_chunks(chunk_count);
        }
    });
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&session_registry)))
            .app_data(web::Data::new(rate_limiter.clone()))
            .app_data(web::Data::new(Arc::clone(&metrics)))
            .wrap(middleware::Logger::default())
            .wrap(middleware::DefaultHeaders::new()
                .add((header::CONTENT_SECURITY_POLICY, "default-src 'self'; style-src 'self' 'unsafe-inline'; script-src 'self' 'unsafe-eval'"))
                .add((header::X_CONTENT_TYPE_OPTIONS, "nosniff"))
                .add((header::X_FRAME_OPTIONS, "DENY")))
            .wrap(rate_limit::RateLimitMiddleware)
            // Register WebSocket route BEFORE catch-all static routes
            .service(handlers::ws_route)
            .service(handlers::health)
            .service(handlers::metrics)
            .service(handlers::session_claim)
            .service(handlers::serve_index)
            .service(handlers::serve_asset)
    })
    .workers(1)
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
