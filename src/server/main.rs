use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::mpsc;
use crate::handlers::AppState;
use crate::world::WorldState;

mod handlers;
mod world;
mod net;
mod db;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // DB pool
    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap_or("postgresql://starve_user:starve_pass@localhost:5432/starve_game".to_string())).await.unwrap();

    // Run migrations
    sqlx::migrate!("./src/db/migrations").run(&pool).await.unwrap();

    // Channels
    let (input_tx, input_rx) = mpsc::unbounded_channel();
    let (snapshot_tx, snapshot_rx) = mpsc::unbounded_channel();

    // Initial world
    let world = WorldState::new();

    // Spawn simulation
    tokio::spawn(run_simulation(input_rx, snapshot_tx, world, pool.clone()));

    // App state
    let app_state = Arc::new(AppState { input_tx, snapshot_rx, pool });

    // Server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .route("/ws", web::get().to(handlers::websocket_handler))
            .route("/static/{filename:.*}", web::get().to(handlers::static_handler))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}