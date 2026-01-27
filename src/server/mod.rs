use actix_web::{web, App, HttpServer, HttpResponse, Responder, HttpRequest};
use actix::Addr;
use crate::game::engine::GameEngine;
use rust_embed::RustEmbed;
use mime_guess::from_path;

mod ws;
pub mod protocol;
pub mod database;

#[derive(RustEmbed)]
#[folder = "src/static"]
struct Asset;

async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

async fn serve_asset(req: HttpRequest) -> impl Responder {
    let path = if req.path() == "/" {
        "index.html"
    } else {
        &req.path()[1..] // Remove leading slash
    };

    match Asset::get(path) {
        Some(content) => {
            let mime = from_path(path).first_or_octet_stream();
            HttpResponse::Ok()
                .content_type(mime.as_ref())
                .insert_header(("Content-Security-Policy", "default-src 'self'; script-src 'self' 'unsafe-eval'; style-src 'self' 'unsafe-inline';"))
                .insert_header(("X-Content-Type-Options", "nosniff"))
                .insert_header(("X-Frame-Options", "DENY"))
                .body(content.data.into_owned())
        }
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

pub async fn start(port: u16, game_addr: Addr<GameEngine>) -> std::io::Result<()> {
    let game_data = web::Data::new(game_addr);
    
    log::info!("Starting HTTP server on port {}", port);
    
    HttpServer::new(move || {
        App::new()
            .app_data(game_data.clone())
            .route("/health", web::get().to(health))
            .route("/ws", web::get().to(ws::ws_route))
            .route("/{filename:.*}", web::get().to(serve_asset))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
