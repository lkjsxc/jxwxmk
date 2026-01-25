use actix::{Actor, StreamHandler, AsyncContext, Handler, Addr, ActorContext, Running};
use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_web_actors::ws;
use uuid::Uuid;
use crate::game::engine::{GameEngine, Join, Leave, Input, WorldUpdate, Craft};
use crate::game::entities::item::ItemType;
use serde::Serialize;

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

#[derive(Serialize)]
struct WelcomeMsg {
    #[serde(rename = "type")]
    msg_type: String,
    id: Uuid,
}

#[derive(Serialize)]
struct WorldMsg<T> {
    #[serde(rename = "type")]
    msg_type: String,
    data: T,
}

impl Actor for GameSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Send Welcome
        let welcome = WelcomeMsg {
            msg_type: "welcome".to_string(),
            id: self.id,
        };
        if let Ok(json) = serde_json::to_string(&welcome) {
            ctx.text(json);
        }

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
    #[serde(default)]
    dx: f64,
    #[serde(default)]
    dy: f64,
    #[serde(default)]
    attack: bool,
    #[serde(default)]
    interact: bool,
    #[serde(default)]
    craft: Option<ItemType>,
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for GameSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                if let Ok(input) = serde_json::from_str::<ClientMessage>(&text) {
                    // Handle Crafting
                    if let Some(item_type) = input.craft {
                         self.game_engine.do_send(Craft {
                             id: self.id,
                             item: item_type,
                         });
                    }

                    // Handle Movement/Actions
                    // Only send if there's actual input to save bandwidth?
                    // For now, always send since client loop sends continuously (or fix client loop)
                    self.game_engine.do_send(Input {
                        id: self.id,
                        dx: input.dx,
                        dy: input.dy,
                        attack: input.attack,
                        interact: input.interact,
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
        let wrapper = WorldMsg {
            msg_type: "world".to_string(),
            data: msg.0,
        };
        if let Ok(json) = serde_json::to_string(&wrapper) {
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