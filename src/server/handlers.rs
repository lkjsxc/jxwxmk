use actix_web::{web, HttpResponse, Responder};
use rust_embed::RustEmbed;
use mime_guess::from_path;

#[derive(RustEmbed)]
#[folder = "src/static"]
struct Asset;

async fn index() -> impl Responder {
    handle_embedded_file("index.html")
}

async fn dist(path: web::Path<String>) -> impl Responder {
    handle_embedded_file(&path.into_inner())
}

fn handle_embedded_file(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => {
            let mime = from_path(path).first_or_octet_stream();
            HttpResponse::Ok()
                .content_type(mime.as_ref())
                .body(content.data.into_owned())
        }
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index))
       .route("/{filename:.*}", web::get().to(dist));
}
