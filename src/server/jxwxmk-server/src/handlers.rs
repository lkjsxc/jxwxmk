use actix_web::{get, post, web, HttpResponse, Result, Error};
use std::sync::Arc;
use crate::logging::StructuredLogger;
use crate::metrics::GameMetrics;

#[get("/health")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

#[get("/metrics")]
pub async fn metrics(metrics: web::Data<Arc<GameMetrics>>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(metrics.render_prometheus())
}

#[post("/session/claim")]
pub async fn session_claim(body: web::Json<serde_json::Value>) -> HttpResponse {
    let player_id = body.get("player_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    
    let token = uuid::Uuid::new_v4().to_string();
    
    StructuredLogger::player_connected(&player_id, &token);
    
    HttpResponse::Ok().json(serde_json::json!({
        "id": player_id,
        "token": token
    }))
}

#[get("/")]
pub async fn serve_index() -> HttpResponse {
    assets::serve_index()
}

#[get("/{filename:.*}")]
pub async fn serve_asset(path: web::Path<String>) -> HttpResponse {
    let filename = path.into_inner();
    assets::serve_asset(&filename)
}

#[get("/ws")]
pub async fn ws_route(
    req: actix_web::HttpRequest,
    stream: web::Payload,
    registry: web::Data<Arc<net::SessionRegistry>>,
) -> Result<HttpResponse, Error> {
    net::ws_route(req, stream, registry).await
}
