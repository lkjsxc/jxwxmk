use actix_web::{HttpResponse, HttpRequest};
use rust_embed::RustEmbed;
use mime_guess::from_path;

#[derive(RustEmbed)]
#[folder = "static"]
pub struct StaticAssets;

pub fn serve_index() -> HttpResponse {
    match StaticAssets::get("index.html") {
        Some(content) => HttpResponse::Ok()
            .content_type("text/html")
            .body(content.data),
        None => HttpResponse::NotFound().body("index.html not found"),
    }
}

pub fn serve_asset(path: &str) -> HttpResponse {
    let path = path.trim_start_matches('/');
    
    match StaticAssets::get(path) {
        Some(content) => {
            let content_type = from_path(path).first_or_octet_stream();
            HttpResponse::Ok()
                .content_type(content_type.as_ref())
                .body(content.data)
        }
        None => HttpResponse::NotFound().body(format!("{} not found", path)),
    }
}

pub fn include_static_assets() {
    // This function ensures the static assets are compiled in
    let _ = StaticAssets::get("index.html");
}
