use actix_web::{web, App, HttpServer};
use actix_web_actors::ws;
use std::env;

mod config;
mod handlers;
mod models;
mod services;
mod utils;

use handlers::websocket::ws_index;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_addr = format!("{}:{}", host, port);

    println!("Starting server at {}", bind_addr);

    HttpServer::new(|| {
        App::new()
            .route("/ws", web::get().to(ws_index))
            .service(
                web::scope("/api")
                    .route("/health", web::get().to(handlers::health::health_check)),
            )
    })
    .bind(&bind_addr)?
    .run()
    .await
}