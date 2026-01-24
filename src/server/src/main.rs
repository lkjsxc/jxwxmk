use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use tokio::sync::{Mutex, broadcast};
use tracing::{info, error};

mod config;
mod network;
mod simulation;
mod world;
mod systems;
mod db;
mod protocol;

use config::ServerConfig;
use simulation::GameSimulation;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("Starting game server...");
    
    // Load configuration
    let config = match ServerConfig::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Config load failed"));
        }
    };
    
    info!("Configuration loaded: {:?}", config);
    
    // Initialize database connection
    let db_pool = match db::create_pool(&config.database.url).await {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to create database pool: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "DB pool creation failed"));
        }
    };
    
    // Clone config before moving parts of it
    let config_clone = config.clone();
    
    // Create broadcast channel
    let (tx, _rx) = broadcast::channel(100);
    
    // Create game simulation
    let simulation = Arc::new(Mutex::new(GameSimulation::new(config.game, tx.clone())));

    // Spawn simulation loop
    let sim_clone = simulation.clone();
    tokio::spawn(async move {
        loop {
            {
                let mut sim = sim_clone.lock().await;
                if !sim.is_running() {
                    sim.start();
                }
                sim.tick().await;
            }
            // Sleep a tiny bit to avoid busy loop if tick returns early
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        }
    });

    // Create shared application state
    let app_state = web::Data::new(AppState {
        config: config_clone,
        simulation,
        db_pool,
        broadcast_sender: tx,
    });
    
    // Start HTTP server
    info!("Starting HTTP server on {}:{}", config.server.host, config.server.port);
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(network::configure_routes)
    })
    .bind((config.server.host, config.server.port))?
    .workers(config.server.workers)
    .run()
    .await
}

#[derive(Clone)]
pub struct AppState {
    pub config: ServerConfig,
    pub simulation: Arc<Mutex<GameSimulation>>,
    pub db_pool: sqlx::PgPool,
    pub broadcast_sender: broadcast::Sender<crate::protocol::ServerMessage>,
}
