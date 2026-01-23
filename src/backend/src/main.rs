use actix_web::{web, App, HttpServer};
use std::env;

mod game;
mod database;
mod websocket;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load environment variables
    dotenv::dotenv().ok();

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    println!("Starting server at {}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .service(websocket::game_websocket)
            .route("/health", web::get().to(|| async { "OK" }))
    })
    .bind((host, port))?
    .run()
    .await
}