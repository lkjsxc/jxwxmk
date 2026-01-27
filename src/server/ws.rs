use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use uuid::Uuid;

use crate::server::session::GameSession;
use crate::server::AppState;

pub async fn ws_route(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let token = req
        .query_string()
        .split('&')
        .find_map(|pair| {
            let mut iter = pair.split('=');
            let key = iter.next()?;
            let value = iter.next()?;
            if key == "token" {
                Uuid::parse_str(value).ok()
            } else {
                None
            }
        });

    let session = GameSession::new(token, data.engine.clone(), data.config.clone());
    ws::start(session, &req, stream)
}
