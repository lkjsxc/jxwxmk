use actix::prelude::*;
use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_web_actors::ws;
use uuid::Uuid;
use crate::game::engine::{GameEngine, Join, Leave, ClientInput};
use crate::server::protocol::{ServerMessage, ClientMessage};

pub struct GameSession {
    id: Uuid,
    token: Option<Uuid>,
    game_addr: Addr<GameEngine>,
}

impl GameSession {
    pub fn new(game_addr: Addr<GameEngine>, token: Option<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(), // Temporary ID, will be updated on Welcome
            token,
            game_addr,
        }
    }
}

impl Actor for GameSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address().recipient();
        self.game_addr.do_send(Join {
            id: self.id,
            token: self.token,
            addr,
        });
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        self.game_addr.do_send(Leave { id: self.id });
    }
}

impl Handler<ServerMessage> for GameSession {
    type Result = ();

    fn handle(&mut self, msg: ServerMessage, ctx: &mut Self::Context) -> Self::Result {
        // Intercept Welcome to update our ID
        if let ServerMessage::Welcome { id, .. } = msg {
            self.id = id;
        }

        if let Ok(json) = serde_json::to_string(&msg) {
            ctx.text(json);
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for GameSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                    self.game_addr.do_send(ClientInput {
                        id: self.id,
                        msg: client_msg,
                    });
                }
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct WsQuery {
    token: Option<Uuid>,
}

pub async fn ws_route(req: HttpRequest, stream: web::Payload, game_addr: web::Data<Addr<GameEngine>>, query: web::Query<WsQuery>) -> Result<HttpResponse, Error> {
    ws::start(GameSession::new(game_addr.get_ref().clone(), query.token), &req, stream)
}
