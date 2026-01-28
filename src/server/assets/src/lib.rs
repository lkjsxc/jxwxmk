use actix_web::HttpResponse;

pub fn serve_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../../../static/index.html"))
}

pub fn serve_asset(filename: &str) -> HttpResponse {
    match filename {
        "styles.css" => HttpResponse::Ok()
            .content_type("text/css")
            .body(include_str!("../../../static/styles.css")),
        "game.js" => HttpResponse::Ok()
            .content_type("application/javascript")
            .body(include_str!("../../../static/game.js")),
        _ => HttpResponse::NotFound().finish(),
    }
}

pub fn get_asset_list() -> Vec<String> {
    vec![
        "index.html".to_string(),
        "styles.css".to_string(),
        "game.js".to_string(),
    ]
}
