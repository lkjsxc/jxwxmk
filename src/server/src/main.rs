use actix::prelude::*;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::{Arc, Mutex};
use tokio::time::{interval, Duration};

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
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://starve:secret@localhost:5432/starve".into());
    let db = PgPool::connect(&database_url).await.expect("DB connect");

    let state = AppState { players: Arc::new(Mutex::new(Vec::new())), db };

    // Spawn a tick to broadcast simple state in background (placeholder)
    let players_clone = state.players.clone();
    actix_web::rt::spawn(async move {
        let mut t = interval(Duration::from_millis(200));
        loop {
            t.tick().await;
            drop(players_clone.lock().unwrap());
                // In a full server we'd broadcast to all websockets; actix actor per-conn needed.
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .service(index)
            .route("/ws/", web::get().to(ws_index))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
