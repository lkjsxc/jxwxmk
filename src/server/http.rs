use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::database;
use crate::server::AppState;
use crate::game::engine::RevokePlayer;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionClaimRequest {
    pub player_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionClaimResponse {
    pub id: String,
    pub token: String,
}

pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

pub async fn session_claim(
    req: HttpRequest,
    body: web::Json<SessionClaimRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let ip = req
        .peer_addr()
        .map(|addr| addr.ip())
        .unwrap_or_else(|| "127.0.0.1".parse().unwrap());
    if !data.session_limiter.allow(ip) {
        return HttpResponse::TooManyRequests().finish();
    }

    let player_id = match Uuid::parse_str(&body.player_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    if let Ok((id, token)) = database::claim_player(&data.db, player_id, &data.config).await {
        data.engine.do_send(RevokePlayer { player_id: id });
        let response = SessionClaimResponse {
            id: id.to_string(),
            token: token.to_string(),
        };
        HttpResponse::Ok().json(response)
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

pub async fn static_index() -> HttpResponse {
    crate::server::static_assets::serve_index()
}

pub async fn static_asset(req: HttpRequest) -> HttpResponse {
    crate::server::static_assets::serve_asset(&req)
}
