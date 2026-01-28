use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use actix_web_actors::ws;
use game::{GameEngine, EngineEvent};
use net::GameSession;
use config::Config;
use persistence::Persistence;

use rust_embed::RustEmbed;
use uuid::Uuid;

#[derive(RustEmbed)]
#[folder = "../static/"]
struct Assets;

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[get("/ws")]
async fn websocket(
    req: HttpRequest,
    stream: web::Payload,
    engine_tx: web::Data<tokio::sync::mpsc::UnboundedSender<EngineEvent>>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = Uuid::new_v4(); 
    ws::start(
        net::GameSession {
            id,
            engine_tx: engine_tx.get_ref().clone(),
        },
        &req,
        stream,
    )
}

fn handle_embedded_file(path: &str) -> HttpResponse {
    match Assets::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(mime_guess::from_path(path).first_or_octet_stream().to_string())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

#[get("/{_:.*}")]
async fn static_assets(path: web::Path<String>) -> impl Responder {
    let mut path = path.into_inner();
    if path.is_empty() || path == "/" {
        path = "index.html".to_string();
    }
    handle_embedded_file(&path)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    log::info!("Starting JXWXMK Server...");

    let config = Config::load_from_dir("../config").expect("Failed to load config");
    
    let _db = Persistence::new(&config.server.db_url).await.expect("Failed to connect to DB");

    let (engine, engine_tx) = GameEngine::new(config.clone());
    
    tokio::spawn(async move {
        engine.run().await;
    });

    let bind_addr = format!("{}:{}", config.server.host, config.server.port);
    log::info!("Listening on http://{}", bind_addr);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(engine_tx.clone()))
            .service(health)
            .service(websocket)
            .service(static_assets)
    })
    .bind(bind_addr)?
    .run()
    .await
}
