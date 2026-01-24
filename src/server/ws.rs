use actix::{Actor, StreamHandler, AsyncContext, Handler, Addr, ActorContext, Running};
use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_web_actors::ws;
use uuid::Uuid;
use crate::game::engine::{GameEngine, Join, Leave, Input, WorldUpdate};

pub struct GameSession {
    id: Uuid,
    game_engine: Addr<GameEngine>,
}

impl GameSession {
    pub fn new(game_engine: Addr<GameEngine>) -> Self {
        Self {
            id: Uuid::new_v4(),
            game_engine,
        }
    }
}

impl Actor for GameSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address().recipient();
        self.game_engine.do_send(Join {
            id: self.id,
            addr,
        });
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.game_engine.do_send(Leave { id: self.id });
        Running::Stop
    }
}

#[derive(serde::Deserialize)]
struct ClientMessage {
    dx: f64,
    dy: f64,
    #[serde(default)]
    attack: bool,
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for GameSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                if let Ok(input) = serde_json::from_str::<ClientMessage>(&text) {
                    self.game_engine.do_send(Input {
                        id: self.id,
                        dx: input.dx,
                        dy: input.dy,
                        attack: input.attack,
                    });
                }
            }
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            _ => (),
        }
    }
}

impl Handler<WorldUpdate> for GameSession {
    type Result = ();
    fn handle(&mut self, msg: WorldUpdate, ctx: &mut Self::Context) {
        if let Ok(json) = serde_json::to_string(&msg.0) {
            ctx.text(json);
        }
    }
}

pub async fn ws_index(
    r: HttpRequest,
    stream: web::Payload,
    game_engine: web::Data<Addr<GameEngine>>,
) -> Result<HttpResponse, Error> {
    ws::start(GameSession::new(game_engine.get_ref().clone()), &r, stream)
}