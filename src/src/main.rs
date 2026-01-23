use actix_files::NamedFile;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize)]
struct Health {
    status: &'static str,
}

#[get("/api/health")]
async fn health() -> impl Responder {
    web::Json(Health { status: "ok" })
}

#[get("/")]
async fn index() -> actix_web::Result<NamedFile> {
    let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "static", "index.html"].iter().collect();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let bind = "0.0.0.0:8080";
    println!("Starting server at http://{}", bind);

    HttpServer::new(|| App::new().service(health).service(index).service(actix_files::Files::new("/static", "static").show_files_listing()))
        .bind(bind)?
        .run()
        .await?;

    Ok(())
}
