use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use tokio::sync::Mutex;
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
    let db_pool = match db::create_pool(&config.database_url).await {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to create database pool: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "DB pool creation failed"));
        }
    };
    
    // Create game simulation
    let simulation = Arc::new(Mutex::new(GameSimulation::new(config.game)));
    
    // Create shared application state
    let app_state = web::Data::new(AppState {
        config: config.clone(),
        simulation,
        db_pool,
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
}