use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_ws::Message as WsMessage;
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::mpsc;
use crate::world::InputEvent;
use crate::net::Message;
use serde::Deserialize;
use serde_json;
use md5;

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

pub type InputTx = mpsc::UnboundedSender<InputEvent>;

#[derive(Clone)]
pub struct AppState {
    pub input_tx: InputTx,
}

pub async fn websocket_handler(
    req: HttpRequest,
    body: web::Payload,
    state: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, actix_web::Error> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    let session_id = "session_123".to_string();  // TODO: proper auth

    // Send join event
    let _ = state.input_tx.send(InputEvent::Join { player_id: session_id.clone() });

    let input_tx = state.input_tx.clone();

    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                WsMessage::Ping(bytes) => {
                    let _ = session.pong(&bytes).await;
                }
                WsMessage::Binary(bytes) => {
                    if let Ok(message) = Message::decode(&bytes) {
                        if message.protocol_version != 1 {
                            let _ = session.close(None).await;
                            return;
                        }
                        // Convert message to InputEvent
                        match message.msg_type {
                            crate::net::MessageType::Input(input_data) => {
                                let event = match input_data.action.as_str() {
                                    "move" => {
                                        if input_data.data.len() >= 8 {
                                            let x = f32::from_le_bytes(input_data.data[0..4].try_into().unwrap());
                                            let y = f32::from_le_bytes(input_data.data[4..8].try_into().unwrap());
                                            InputEvent::Move { player_id: input_data.player_id, x, y }
                                        } else {
                                            continue;
                                        }
                                    }
                                    "gather" => {
                                        if !input_data.data.is_empty() {
                                            let node_id = input_data.data[0] as u32;
                                            InputEvent::Gather { player_id: input_data.player_id, node_id }
                                        } else {
                                            continue;
                                        }
                                    }
                                    "craft" => {
                                        if !input_data.data.is_empty() {
                                            InputEvent::Craft { player_id: input_data.player_id, recipe_id: input_data.data[0] }
                                        } else {
                                            continue;
                                        }
                                    }
                                    "consume" => {
                                        if !input_data.data.is_empty() {
                                            InputEvent::Consume { player_id: input_data.player_id, food_id: input_data.data[0] as u8 }
                                        } else {
                                            continue;
                                        }
                                    }
                                    _ => continue,
                                };
                                let _ = input_tx.send(event);
                            }
                            _ => {}
                        }
                    }
                }
                WsMessage::Close(_) => {
                    let _ = session.close(None).await;
                    return;
                }
                _ => {}
            }
        }
    });

    // For now, no snapshot sending

    Ok(response)
}

pub async fn login_handler(
    req: web::Json<LoginRequest>,
    _state: web::Data<Arc<AppState>>,
) -> impl Responder {
    // Placeholder auth
    if req.username == "test" && req.password == "pass" {
        HttpResponse::Ok().json(serde_json::json!({ "session_id": "session_123" }))
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

pub async fn index_handler() -> impl Responder {
    match tokio::fs::read("/app/static/index.html").await {
        Ok(content) => HttpResponse::Ok().content_type("text/html; charset=utf-8").body(content),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn static_handler(path: web::Path<String>) -> impl Responder {
    let filename = path.into_inner();
    // Serve from /app/static
    let content_type = if filename.ends_with(".js") {
        "application/javascript; charset=utf-8"
    } else if filename.ends_with(".html") {
        "text/html; charset=utf-8"
    } else if filename.ends_with(".css") {
        "text/css; charset=utf-8"
    } else {
        "text/plain; charset=utf-8"
    };
    
    match tokio::fs::read(format!("/app/static/{}", filename)).await {
        Ok(content) => {
            let mut response = HttpResponse::Ok().content_type(content_type);
            
            // Add cache busting headers for JS/CSS
            if filename.ends_with(".js") || filename.ends_with(".css") {
                response = response
                    .insert_header(("Cache-Control", "public, max-age=31536000, immutable"))
                    .insert_header(("ETag", format!("{:x}", md5::compute(&content))));
            } else {
                response = response.insert_header(("Cache-Control", "no-cache"));
            }
            
            response.body(content)
        },
        Err(_) => HttpResponse::NotFound().finish(),
    }
}