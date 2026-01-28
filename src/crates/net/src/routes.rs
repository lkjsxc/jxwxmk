use actix_web::{web, HttpResponse, Responder, HttpRequest};
use assets::{get_asset, get_index};
use crate::ServerState;
use serde::Deserialize;
use uuid::Uuid;
use game::GetMetrics;

#[derive(Deserialize)]
pub struct ClaimRequest {
    pub player_id: Uuid,
}

#[derive(Deserialize)]
pub struct WsQuery {
    pub token: Uuid,
}

pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

pub async fn metrics(data: web::Data<ServerState>) -> impl Responder {
    match data.game_addr.send(GetMetrics).await {
        Ok(m) => HttpResponse::Ok().body(m),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn index() -> impl Responder {
    match get_index() {
        Some((data, mime)) => HttpResponse::Ok().content_type(mime).body(data),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

pub async fn static_asset(path: web::Path<String>) -> impl Responder {
    match get_asset(&path) {
        Some((data, mime)) => HttpResponse::Ok().content_type(mime).body(data),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

pub async fn claim_session(
    data: web::Data<ServerState>,
    req: HttpRequest,
    claim: web::Json<ClaimRequest>,
) -> impl Responder {
    let ip = req.connection_info().realip_remote_addr()
        .unwrap_or("unknown")
        .to_string();

    {
        let mut limiter = data.rate_limiter.lock().unwrap();
        if !limiter.check(ip) {
            return HttpResponse::TooManyRequests().body("Rate limit exceeded");
        }
    }

    match data.persistence.claim_session(claim.player_id).await {
        Ok(token) => HttpResponse::Ok().json(serde_json::json!({ 
            "id": claim.player_id,
            "token": token 
        })),
        Err(e) => {
            eprintln!("Failed to claim session: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn ws_index(
    req: actix_web::HttpRequest,
    stream: web::Payload,
    query: web::Query<WsQuery>,
    data: web::Data<ServerState>,
) -> Result<HttpResponse, actix_web::Error> {
    actix_web_actors::ws::start(
        crate::ws_actor::WsSession::new(
            data.game_addr.clone(), 
            data.persistence.get_pool().clone(),
            query.token
        ), 
        &req, 
        stream
    )
}