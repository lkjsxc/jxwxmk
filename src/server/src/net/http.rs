use actix_web::{body::BoxBody, web, App, HttpRequest, HttpResponse, Responder};
use mime_guess::from_path;
use rust_embed::RustEmbed;
use uuid::Uuid;

use crate::config::{SessionClaimRequest, SessionClaimResponse};
use crate::game::RevokeSession;
use crate::net::security::security_headers;
use crate::net::ws::ws_index;
use crate::net::AppState;
use crate::persistence::{load_player_by_id, rotate_token};

#[derive(RustEmbed)]
#[folder = "../static"]
struct StaticAssets;

pub fn build_app(
    state: AppState,
) -> App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse<BoxBody>,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new()
        .app_data(web::Data::new(state))
        .wrap(security_headers())
        .route("/health", web::get().to(health))
        .route("/session/claim", web::post().to(session_claim))
        .route("/ws", web::get().to(ws_index))
        .route("/", web::get().to(serve_index))
        .route("/{filename:.*}", web::get().to(serve_asset))
}

async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

async fn session_claim(
    req: HttpRequest,
    payload: web::Json<SessionClaimRequest>,
    data: web::Data<AppState>,
) -> actix_web::Result<HttpResponse> {
    let ip = req
        .peer_addr()
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let mut limiter = data.claim_limiter.lock().expect("rate limiter lock");
    if !limiter.allow(&ip) {
        return Ok(HttpResponse::TooManyRequests().finish());
    }

    let player_id = payload.player_id;
    let existing = load_player_by_id(&data.db, player_id, &data.config).await;
    if existing.is_none() {
        return Ok(HttpResponse::BadRequest().body("invalid_player_id"));
    }

    let token = Uuid::new_v4();
    rotate_token(&data.db, player_id, token).await.map_err(|err| {
        actix_web::error::ErrorInternalServerError(format!("db_error: {err}"))
    })?;

    data.engine.do_send(RevokeSession {
        player_id,
        reason: "login_elsewhere".to_string(),
    });

    Ok(HttpResponse::Ok().json(SessionClaimResponse { id: player_id, token }))
}

async fn serve_index() -> actix_web::Result<HttpResponse> {
    serve_asset_inner("index.html")
}

async fn serve_asset(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let path = req.match_info().query("filename");
    if path.is_empty() {
        return serve_asset_inner("index.html");
    }
    serve_asset_inner(path)
}

fn serve_asset_inner(path: &str) -> actix_web::Result<HttpResponse> {
    if let Some(content) = StaticAssets::get(path) {
        let body: Vec<u8> = content.data.into();
        let mime = from_path(path).first_or_octet_stream();
        Ok(HttpResponse::Ok()
            .content_type(mime.as_ref())
            .body(body))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}
