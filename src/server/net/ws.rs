use actix::{Actor, StreamHandler, AsyncContext, Addr};
use actix_web_actors::ws;
use actix_web::{web, HttpRequest, HttpResponse, Error};
use uuid::Uuid;
use crate::game::GameEngine;
use crate::game::messages::{ClientConnected, ClientDisconnected, ClientRequest};
use crate::protocol::{ClientMessage, ServerMessage};

pub struct GameSession {
    id: Uuid,
    // token: Uuid,
    game_engine: Addr<GameEngine>,
}

impl GameSession {
    pub fn new(id: Uuid, game_engine: Addr<GameEngine>) -> Self {
        Self { id, game_engine }
    }
}

impl Actor for GameSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.game_engine.do_send(ClientConnected {
            id: self.id,
            addr: addr.recipient(),
        });
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::Running {
        self.game_engine.do_send(ClientDisconnected { id: self.id });
        actix::Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for GameSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                match serde_json::from_str::<ClientMessage>(&text) {
                    Ok(client_msg) => {
                         self.game_engine.do_send(ClientRequest {
                             id: self.id,
                             msg: client_msg,
                         });
                    },
                    Err(e) => {
                        log::warn!("Invalid message: {}", e);
                    }
                }
            }
            _ => (),
        }
    }
}

impl actix::Handler<ServerMessage> for GameSession {
    type Result = ();

    fn handle(&mut self, msg: ServerMessage, ctx: &mut Self::Context) {
        if let Ok(json) = serde_json::to_string(&msg) {
            ctx.text(json);
        }
    }
}

#[derive(serde::Deserialize)]
pub struct WsQuery {
    token: Uuid,
    // Optional id for testing/mocking until DB lookup is real
    id: Option<Uuid>, 
}

pub async fn ws_route(
    req: HttpRequest,
    stream: web::Payload,
    engine: web::Data<Addr<GameEngine>>,
    query: web::Query<WsQuery>,
) -> Result<HttpResponse, Error> {
    // TODO: Look up player ID from token in DB
    // For now, assume ID is passed or generate random if not (for testing)
    let player_id = query.id.unwrap_or_else(Uuid::new_v4);
    
    let session = GameSession::new(player_id, engine.get_ref().clone());
    ws::start(session, &req, stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    use crate::game::GameEngine;
    use actix_test;

    #[actix_web::test]
    async fn test_ws_connection() {
        let engine = GameEngine::new().start();
        
        let srv = actix_test::start(move || {
            App::new()
                .app_data(web::Data::new(engine.clone()))
                .route("/ws", web::get().to(ws_route))
        });

        let id = Uuid::new_v4();
        let token = Uuid::new_v4();
        
        let mut client = awc::Client::new();
        let (_resp, _connection) = client
            .ws(srv.url(&format!("/ws?token={}&id={}", token, id)))
            .connect()
            .await
            .expect("Failed to connect");
            
        // If we connected, handshake passed.
    }
}
