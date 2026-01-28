use actix_web::{web, App, HttpResponse, HttpServer, middleware, http::header};
use std::sync::Arc;

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    log::info!("Starting JXWXMK server...");
    
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::DefaultHeaders::new()
                .add((header::CONTENT_SECURITY_POLICY, "default-src 'self'; style-src 'self' 'unsafe-inline'; script-src 'self' 'unsafe-eval'"))
                .add((header::X_CONTENT_TYPE_OPTIONS, "nosniff"))
                .add((header::X_FRAME_OPTIONS, "DENY")))
            .service(handlers::health)
            .service(handlers::metrics)
            .service(handlers::session_claim)
            .service(handlers::serve_index)
            .service(handlers::serve_asset)
            .service(handlers::ws_route)
    })
    .workers(1)
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
