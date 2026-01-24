use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_ws::{Message, Session};
use futures::StreamExt;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::mpsc;
use crate::world::{WorldState, run_simulation};
use crate::net::{InputEvent, SnapshotEvent, Message};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

pub type InputTx = mpsc::UnboundedSender<InputEvent>;
pub type SnapshotRx = mpsc::UnboundedReceiver<SnapshotEvent>;

#[derive(Clone)]
pub struct AppState {
    pub input_tx: InputTx,
    pub snapshot_rx: SnapshotRx,
    pub pool: PgPool,
}

pub async fn websocket_handler(
    req: HttpRequest,
    body: web::Payload,
    state: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, actix_web::Error> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    let session_id = "session_123".to_string();  // TODO: proper auth

    let input_tx = state.input_tx.clone();
    let mut snapshot_rx = state.snapshot_rx.clone();

    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Ping(bytes) => {
                    let _ = session.pong(&bytes).await;
                }
                Message::Binary(bytes) => {
                    if let Ok(message) = Message::decode(&bytes) {
                        if message.protocol_version != 1 {
                            let _ = session.close(None).await;
                            return;
                        }
                        let _ = input_tx.send(InputEvent {
                            session_id: session_id.clone(),
                            message,
                        });
                    }
                }
                Message::Close(_) => {
                    let _ = session.close(None).await;
                    return;
                }
                _ => {}
            }
        }
    });

    actix_web::rt::spawn(async move {
        while let Some(snapshot) = snapshot_rx.recv().await {
            if snapshot.session_id == session_id {
                if let Ok(bytes) = snapshot.message.encode() {
                    let _ = session.binary(bytes).await;
                }
            }
        }
    });

    Ok(response)
}

pub async fn login_handler(
    req: web::Json<LoginRequest>,
    state: web::Data<Arc<AppState>>,
) -> impl Responder {
    // Placeholder auth
    if req.username == "test" && req.password == "pass" {
        HttpResponse::Ok().json(serde_json::json!({ "session_id": "session_123" }))
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

pub async fn static_handler(path: web::Path<String>) -> impl Responder {
    let filename = path.into_inner();
    // Serve from /app/static
    let content_type = if filename.ends_with(".js") {
        "application/javascript"
    } else if filename.ends_with(".html") {
        "text/html"
    } else {
        "text/plain"
    };
    match tokio::fs::read(format!("/app/static/{}", filename)).await {
        Ok(content) => HttpResponse::Ok().content_type(content_type).body(content),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn run_simulation(
    mut input_rx: mpsc::UnboundedReceiver<InputEvent>,
    snapshot_tx: mpsc::UnboundedSender<SnapshotEvent>,
    mut world: WorldState,
    pool: sqlx::PgPool,
) {
    use tokio::time::{interval, Duration};
    let mut interval = interval(Duration::from_millis(50));
    let mut tick = 0u64;

    loop {
        interval.tick().await;

        while let Ok(input) = input_rx.try_recv() {
            world.process_input(&input);
        }

        world.tick(&pool).await;

        for session_id in world.get_sessions() {
            let snapshot = world.create_snapshot(tick);
            let _ = snapshot_tx.send(SnapshotEvent {
                session_id: session_id.clone(),
                message: snapshot,
            });
        }

        tick += 1;
    }
}