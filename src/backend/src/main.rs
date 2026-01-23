use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};
use std::env;

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("{}:{}", host, port);

    log::info!("Starting server at http://{}", addr);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(health_check)
    })
    .bind(addr)?
    .run()
    .await
}
