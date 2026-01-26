use actix_web::{web, HttpResponse, HttpRequest, Responder};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "static/"]
struct Asset;

pub async fn serve_asset(path: web::Path<String>, _req: HttpRequest) -> impl Responder {
    let path_str = path.into_inner();
    let file_path = if path_str.is_empty() { "index.html" } else { &path_str };

    match Asset::get(file_path) {
        Some(content) => {
            let mime = mime_guess::from_path(file_path).first_or_octet_stream();
            HttpResponse::Ok()
                .content_type(mime.as_ref())
                .body(content.data.into_owned())
        }
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

pub async fn serve_index() -> impl Responder {
    match Asset::get("index.html") {
        Some(content) => {
            HttpResponse::Ok()
                .content_type("text/html")
                .body(content.data.into_owned())
        }
        None => HttpResponse::NotFound().body("index.html not found"),
    }
}
