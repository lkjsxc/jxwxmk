use actix_files::Files;
use actix_web::{get, web, HttpResponse, Responder};

use crate::app::ws::ws_index;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health)
        .service(web::resource("/ws").route(web::get().to(ws_index)))
        .service(Files::new("/static", "./static").index_file("index.html"));
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("ok")
}
