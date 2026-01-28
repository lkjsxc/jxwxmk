mod config;
mod protocol;
mod handlers;
mod persistence;
mod game;

use actix_web::{App, HttpServer, middleware};
use env_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    log::info!("Starting server...");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(handlers::config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
