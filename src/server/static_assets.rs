use actix_web::{http::header, HttpRequest, HttpResponse};
use mime_guess::from_path;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "static"]
struct Assets;

pub fn serve_index() -> HttpResponse {
    serve_asset_path("index.html")
}

pub fn serve_asset(req: &HttpRequest) -> HttpResponse {
    let path = req.match_info().query("filename");
    if path.is_empty() {
        return serve_index();
    }
    serve_asset_path(path)
}

fn serve_asset_path(path: &str) -> HttpResponse {
    match Assets::get(path) {
        Some(content) => {
            let body = content.data.into_owned();
            let mime = from_path(path).first_or_octet_stream();
            HttpResponse::Ok()
                .insert_header((header::CONTENT_TYPE, mime.as_ref()))
                .body(body)
        }
        None => HttpResponse::NotFound().finish(),
    }
}
