use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::PgPool;
use rust_embed::{RustEmbed, Embed};
use mime_guess;

#[derive(Deserialize)]
pub struct ClaimRequest {
    pub player_id: Uuid,
}

#[derive(Serialize)]
pub struct ClaimResponse {
    pub id: Uuid,
    pub token: Uuid,
}

pub async fn claim_session(
    _pool: web::Data<PgPool>, // TODO: Use DB
    req: web::Json<ClaimRequest>,
) -> impl Responder {
    // Mock implementation for now
    let token = Uuid::new_v4();
    HttpResponse::Ok().json(ClaimResponse {
        id: req.player_id,
        token,
    })
}

#[derive(RustEmbed)]
#[folder = "../static"]
pub struct Assets;

pub async fn serve_asset(path: web::Path<String>) -> HttpResponse {
    let path = path.into_inner();
    match Assets::get(&path) {
        Some(content) => {
            let mime = mime_guess::from_path(&path).first_or_octet_stream();
            HttpResponse::Ok()
                .content_type(mime.as_ref())
                .body(content.data.into_owned())
        }
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

pub async fn serve_index() -> HttpResponse {
    match Assets::get("index.html") {
        Some(content) => {
             HttpResponse::Ok()
                .content_type("text/html")
                .body(content.data.into_owned())
        }
        None => HttpResponse::NotFound().body("index.html not found"),
    }
}
