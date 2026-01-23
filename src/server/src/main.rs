use actix::prelude::*;
use actix_files::Files;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

#[derive(Clone, Serialize, Deserialize)]
struct Player {
    id: String,
    x: f32,
    y: f32,
}

#[derive(Clone)]
struct AppState {
    players: Arc<Mutex<Vec<Player>>>,
    db: PgPool,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("StarveRS server running")
}

async fn ws_index(req: HttpRequest, stream: web::Payload, data: Data<AppState>) -> actix_web::Result<HttpResponse> {
    let resp = ws::start(MyWs { state: data.get_ref().clone() }, &req, stream);
    resp
}

struct MyWs {
    state: AppState,
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(ws::Message::Text(txt)) => {
                if let Ok(p) = serde_json::from_str::<Player>(&txt) {
                    let mut players = self.state.players.lock().unwrap();
                    if let Some(existing) = players.iter_mut().find(|pl| pl.id == p.id) {
                        existing.x = p.x;
                        existing.y = p.y;
                    } else {
                        players.push(p);
                    }
                }
            }
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            _ => (),
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://starve:secret@db:5432/starve".into());

    // retry connecting to the DB a few times for container startup ordering
    let mut tries = 0u8;
    let db: PgPool;
    loop {
        match PgPool::connect(&database_url).await {
            Ok(pool) => { db = pool; break; }
            Err(e) => {
                tries += 1;
                if tries > 20 { panic!("DB connect: {}", e); }
                eprintln!("DB connect failed (attempt {}): {}", tries, e);
                sleep(Duration::from_secs(1)).await;
            }
        }
    }

    let state = AppState { players: Arc::new(Mutex::new(Vec::new())), db };

    // Background tick (placeholder)
    let players_clone = state.players.clone();
    actix_web::rt::spawn(async move {
        loop {
            sleep(Duration::from_millis(200)).await;
            drop(players_clone.lock().unwrap());
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .service(Files::new("/", "/usr/local/share/starve/static").index_file("index.html"))
            .service(index)
            .route("/ws/", web::get().to(ws_index))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
