use std::sync::Arc;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::interval;
use tokio::sync::{RwLock, mpsc};
use uuid::Uuid;

use config::GameConfig;
use game::{GameEngine, GameHandle, GameResponse};
use net::run_server;
use persistence::PersistenceHandle;
use protocol::ServerMessage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Load configuration
    let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "/app/config".to_string());
    
    log::info!("Loading configuration from {}", config_path);
    
    let config = match GameConfig::load_from_dir(&config_path) {
        Ok(c) => {
            log::info!("Configuration loaded successfully");
            Arc::new(c)
        }
        Err(e) => {
            log::error!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    // Connect to database
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres@localhost/jxwxmk".to_string());
    
    log::info!("Connecting to database");
    
    let persistence = match PersistenceHandle::new(&database_url).await {
        Ok(p) => {
            log::info!("Database connected");
            
            // Run migrations
            if let Err(e) = p.migrate().await {
                log::error!("Migration failed: {}", e);
                std::process::exit(1);
            }
            
            p
        }
        Err(e) => {
            log::error!("Database connection failed: {}", e);
            std::process::exit(1);
        }
    };

    // Create game engine
    let engine = GameEngine::new(Arc::clone(&config));
    let game_handle = GameHandle::new(engine);

    // Create sessions map (shared between server and tick loop)
    let sessions: Arc<RwLock<HashMap<Uuid, mpsc::UnboundedSender<ServerMessage>>>> = 
        Arc::new(RwLock::new(HashMap::new()));

    // Start tick loop
    let tick_rate = config.server.tick_rate;
    let dt = 1.0 / tick_rate as f64;
    let game_for_tick = game_handle.clone();
    let sessions_for_tick = sessions.clone();
    
    tokio::spawn(async move {
        let mut tick_interval = interval(Duration::from_secs_f64(dt));
        
        loop {
            tick_interval.tick().await;
            let responses = game_for_tick.tick(dt).await;
            
            // Send responses to connected sessions
            let sessions = sessions_for_tick.read().await;
            for response in responses {
                match response {
                    GameResponse::PlayerUpdate { player_id, message } |
                    GameResponse::ToPlayer { player_id, message } => {
                        if let Some(tx) = sessions.get(&player_id) {
                            let _ = tx.send(message);
                        }
                    }
                    GameResponse::Error { player_id, error } => {
                        if let Some(tx) = sessions.get(&player_id) {
                            let _ = tx.send(ServerMessage::Error { data: error });
                        }
                    }
                    _ => {}
                }
            }
        }
    });

    // Start HTTP/WebSocket server
    let bind_addr = config.server.bind_http.clone();
    let ws_config = net::WsSessionConfig {
        heartbeat_interval: Duration::from_secs(config.server.limits.ws_heartbeat_interval_secs),
        client_timeout: Duration::from_secs(config.server.limits.ws_idle_timeout_secs),
    };
    
    log::info!("Starting server on {}", bind_addr);
    
    run_server(&bind_addr, game_handle, persistence, sessions, ws_config).await
}
