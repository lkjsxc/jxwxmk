use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest};
use actix_web::middleware::{Logger, DefaultHeaders};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::{RwLock, mpsc};
use uuid::Uuid;

use game::GameHandle;
use persistence::PersistenceHandle;
use protocol::*;
use assets::*;

use crate::session::{SessionsMap, ws_handler};
use crate::metrics::Metrics;

pub struct ServerState {
    pub game: GameHandle,
    pub persistence: PersistenceHandle,
    pub sessions: SessionsMap,
    pub metrics: Metrics,
}

pub async fn run_server(
    bind_addr: &str,
    game: GameHandle,
    persistence: PersistenceHandle,
    sessions: SessionsMap,
) -> std::io::Result<()> {
    let state = Arc::new(ServerState {
        game,
        persistence,
        sessions,
        metrics: Metrics::new(),
    });

    log::info!("Starting server on {}", bind_addr);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.game.clone()))
            .app_data(web::Data::new(state.persistence.clone()))
            .app_data(web::Data::new(state.sessions.clone()))
            .app_data(web::Data::new(state.metrics.clone()))
            .wrap(Logger::default())
            .wrap(DefaultHeaders::new()
                .add(("X-Content-Type-Options", "nosniff"))
                .add(("X-Frame-Options", "DENY"))
                .add(("Content-Security-Policy", "default-src 'self'; script-src 'self' 'unsafe-eval'; style-src 'self' 'unsafe-inline'"))
            )
            .route("/health", web::get().to(health_handler))
            .route("/metrics", web::get().to(metrics_handler))
            .route("/session/claim", web::post().to(session_claim_handler))
            .route("/ws", web::get().to(ws_route))
            .route("/", web::get().to(index_handler))
            .route("/{filename:.*}", web::get().to(asset_handler))
    })
    .bind(bind_addr)?
    .workers(1)
    .run()
    .await
}

async fn health_handler() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

async fn metrics_handler(metrics: web::Data<Metrics>) -> HttpResponse {
    let metrics_text = metrics.gather();
    HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4")
        .body(metrics_text)
}

async fn session_claim_handler(
    persistence: web::Data<PersistenceHandle>,
    sessions: web::Data<SessionsMap>,
    body: web::Json<SessionClaimRequest>,
) -> HttpResponse {
    let player_id = body.player_id;
    
    // Rotate token in database
    match persistence.rotate_token(player_id).await {
        Ok(new_token) => {
            // Revoke existing session if any
            let sessions_read = sessions.read().await;
            if let Some(tx) = sessions_read.get(&player_id) {
                let _ = tx.send(ServerMessage::SessionRevoked {
                    reason: "login_elsewhere".to_string(),
                });
            }
            drop(sessions_read);

            let response = SessionClaimResponse {
                id: player_id,
                token: new_token,
            };
            
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            log::error!("Failed to rotate token: {}", e);
            HttpResponse::InternalServerError().body("Database error")
        }
    }
}

async fn ws_route(
    req: HttpRequest,
    body: web::Payload,
    game: web::Data<GameHandle>,
    persistence: web::Data<PersistenceHandle>,
    sessions: web::Data<SessionsMap>,
) -> Result<HttpResponse, actix_web::Error> {
    ws_handler(req, body, game, persistence, sessions).await
}

async fn index_handler() -> HttpResponse {
    assets::serve_index()
}

async fn asset_handler(req: HttpRequest) -> HttpResponse {
    let filename: String = req.match_info().query("filename").parse().unwrap_or_default();
    assets::serve_asset(&filename)
}
