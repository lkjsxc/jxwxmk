mod server;
mod game;
mod db;

use actix_web::{App, HttpServer, web, middleware};
use actix_files as fs;
use actix::Actor;
use log::info;
use game::engine::GameEngine;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("Starting kkmypk Server...");

    let game_engine = GameEngine::new().start();
    let game_engine_data = web::Data::new(game_engine);

    HttpServer::new(move || {
        App::new()
            .app_data(game_engine_data.clone())
            .wrap(middleware::Logger::default())
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(("Content-Security-Policy", "default-src 'self'; script-src 'self' 'unsafe-eval'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; connect-src 'self' ws: wss:;"))
                    .add(("X-Content-Type-Options", "nosniff"))
                    .add(("X-Frame-Options", "DENY"))
            )
            .service(server::http::health_check)
            .route("/ws", web::get().to(server::ws::ws_index))
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}