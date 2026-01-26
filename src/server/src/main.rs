mod app;
mod world;

use actix_web::{App, HttpServer};
use app::{config::AppConfig, http::configure_routes, AppState};
use world::WorldRunner;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::from_env();
    let (input_tx, snapshot_tx) = WorldRunner::start(config.tick_hz);

    let state = AppState::new(input_tx, snapshot_tx, config.tick_hz);

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(state.clone()))
            .configure(configure_routes)
    })
    .bind(config.bind_address())?
    .run()
    .await
}
