use actix_web::{get, post, web, HttpResponse, Result, Error};
use actix_web_actors::ws;
use std::sync::Arc;

#[get("/health")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

#[get("/metrics")]
pub async fn metrics() -> HttpResponse {
    let metrics_text = r#"# HELP jxwxmk_tick_duration_seconds Tick duration
# TYPE jxwxmk_tick_duration_seconds histogram
jxwxmk_tick_duration_seconds_bucket{le="0.01"} 0
jxwxmk_tick_duration_seconds_bucket{le="0.1"} 0
jxwxmk_tick_duration_seconds_bucket{le="+Inf"} 0
jxwxmk_tick_duration_seconds_sum 0
jxwxmk_tick_duration_seconds_count 0

# HELP jxwxmk_active_players Number of active players
# TYPE jxwxmk_active_players gauge
jxwxmk_active_players 0

# HELP jxwxmk_active_chunks Number of active chunks
# TYPE jxwxmk_active_chunks gauge
jxwxmk_active_chunks 0
"#;
    HttpResponse::Ok().content_type("text/plain").body(metrics_text)
}

#[post("/session/claim")]
pub async fn session_claim(body: web::Json<serde_json::Value>) -> HttpResponse {
    let player_id = body.get("player_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    
    let token = uuid::Uuid::new_v4().to_string();
    
    HttpResponse::Ok().json(serde_json::json!({
        "id": player_id,
        "token": token
    }))
}

#[get("/")]
pub async fn serve_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../../static/index.html"))
}

#[get("/{filename:.*}")]
pub async fn serve_asset(path: web::Path<String>) -> HttpResponse {
    let filename = path.into_inner();
    match filename.as_str() {
        "styles.css" => HttpResponse::Ok()
            .content_type("text/css")
            .body(include_str!("../../static/styles.css")),
        "game.js" => HttpResponse::Ok()
            .content_type("application/javascript")
            .body(include_str!("../../static/game.js")),
        _ => HttpResponse::NotFound().finish(),
    }
}

#[get("/ws")]
pub async fn ws_route(req: actix_web::HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(WsSession::new(), &req, stream)
}

use actix::prelude::*;
use actix_web_actors::ws::{WebsocketContext, Message as WsMessage, ProtocolError};
use std::time::Instant;

pub struct WsSession {
    hb: Instant,
    player_id: uuid::Uuid,
    token: uuid::Uuid,
}

impl WsSession {
    fn new() -> Self {
        WsSession {
            hb: Instant::now(),
            player_id: uuid::Uuid::new_v4(),
            token: uuid::Uuid::new_v4(),
        }
    }
}

impl Actor for WsSession {
    type Context = WebsocketContext<Self>;
    
    fn started(&mut self, ctx: &mut Self::Context) {
        let welcome = serde_json::json!({
            "type": "welcome",
            "id": self.player_id.to_string(),
            "token": self.token.to_string(),
            "version": 3,
            "spawned": false
        });
        ctx.text(welcome.to_string());
    }
}

impl StreamHandler<Result<WsMessage, ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<WsMessage, ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(WsMessage::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(WsMessage::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(WsMessage::Text(text)) => {
                handle_message(self, &text, ctx);
            }
            Ok(WsMessage::Close(_)) => {
                ctx.stop();
            }
            _ => (),
        }
    }
}

fn handle_message(
    session: &mut WsSession,
    text: &str,
    ctx: &mut WebsocketContext<WsSession>,
) {
    match serde_json::from_str::<serde_json::Value>(text) {
        Ok(msg) => {
            let msg_type = msg.get("type").and_then(|v| v.as_str());
            
            match msg_type {
                Some("spawn") => {
                    let player_update = serde_json::json!({
                        "type": "playerUpdate",
                        "data": {
                            "id": session.player_id.to_string(),
                            "name": "Player",
                            "spawned": true,
                            "vitals": {
                                "hp": 30.0,
                                "max_hp": 30.0,
                                "hunger": 80.0,
                                "max_hunger": 100.0,
                                "temperature": 50.0,
                                "max_temperature": 100.0
                            },
                            "inventory": [null; 30],
                            "active_slot": 0,
                            "level": 1,
                            "xp": 0,
                            "stats": {
                                "steps": 0,
                                "kills": 0,
                                "crafts": 0,
                                "gathers": 0,
                                "deaths": 0
                            },
                            "quests": [],
                            "achievements": []
                        }
                    });
                    ctx.text(player_update.to_string());
                }
                _ => {
                    let notification = serde_json::json!({
                        "type": "notification",
                        "data": { "text": "Message received" }
                    });
                    ctx.text(notification.to_string());
                }
            }
        }
        Err(_) => {
            let error = serde_json::json!({
                "type": "error",
                "data": {
                    "code": "invalid_message",
                    "message": "Invalid message format",
                    "details": null
                }
            });
            ctx.text(error.to_string());
        }
    }
}
