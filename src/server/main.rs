use actix_web::{web, App, HttpServer, middleware};
use std::sync::Arc;
use tokio::sync::mpsc;
use crate::handlers::AppState;
use crate::world::{WorldState, run_simulation};

mod handlers;
mod world;
mod net;
mod db;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Channels
    let (input_tx, input_rx) = mpsc::unbounded_channel();
    let (snapshot_tx, _) = mpsc::unbounded_channel();

    // Initial world
    let world = WorldState::new();

    // Spawn simulation
    tokio::spawn(run_simulation(input_rx, snapshot_tx, world));

    // App state
    let app_state = Arc::new(AppState { input_tx });

    // Server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(middleware::DefaultHeaders::new()
                .add(("Content-Security-Policy", "default-src 'self'; script-src 'self' 'unsafe-eval'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; connect-src 'self' ws: wss:"))
                .add(("X-Content-Type-Options", "nosniff"))
                .add(("X-Frame-Options", "DENY"))
                .add(("X-XSS-Protection", "1; mode=block"))
                .add(("Referrer-Policy", "strict-origin-when-cross-origin")))
            .route("/", web::get().to(handlers::index_handler))
            .route("/login", web::post().to(handlers::login_handler))
            .route("/ws", web::get().to(handlers::websocket_handler))
            .route("/static/{filename:.*}", web::get().to(handlers::static_handler))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}