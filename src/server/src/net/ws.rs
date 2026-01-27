use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use uuid::Uuid;

use crate::net::session::GameSession;
use crate::net::AppState;
use crate::persistence::{load_player_by_token, save_player};
use crate::game::PlayerState;

pub async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let token = parse_token(req.query_string());
    let mut player_id = None;
    let mut session_token = None;

    if let Some(token) = token {
        if let Some(player) = load_player_by_token(&data.db, token, &data.config).await {
            player_id = Some(player.id);
            session_token = Some(player.token);
        }
    }

    let is_new = player_id.is_none();
    let player_id = player_id.unwrap_or_else(Uuid::new_v4);
    let session_token = session_token.unwrap_or_else(Uuid::new_v4);

    if is_new {
        let player = PlayerState::new(
            player_id,
            session_token,
            data.config.balance.player.inventory_slots,
            data.config.balance.player.max_health,
        );
        let _ = save_player(&data.db, &player).await;
    }

    let session = GameSession::new(player_id, session_token, data.config.clone(), data.engine.clone());
    ws::start(session, &req, stream)
}

fn parse_token(query: &str) -> Option<Uuid> {
    for part in query.split('&') {
        let mut iter = part.splitn(2, '=');
        let key = iter.next()?;
        let value = iter.next().unwrap_or("");
        if key == "token" {
            if let Ok(token) = Uuid::parse_str(value) {
                return Some(token);
            }
        }
    }
    None
}
