use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;
use uuid::Uuid;

use crate::config::Config;
use crate::game::engine::{
    AcceptQuestMsg, CraftMsg, InputMsg, Join, Leave, NameMsg, NpcActionMsg, SlotMsg, SpawnMsg,
    SwapSlotsMsg, TradeMsg,
};
use crate::protocol::client::ClientMessage;
use crate::protocol::server::ServerMessage;

pub struct GameSession {
    session_id: Uuid,
    player_id: Option<Uuid>,
    token: Option<Uuid>,
    engine: Addr<crate::game::engine::GameEngine>,
    config: Config,
    last_window: Instant,
    msg_count: u32,
    abuse_strikes: u32,
}

impl GameSession {
    pub fn new(token: Option<Uuid>, engine: Addr<crate::game::engine::GameEngine>, config: Config) -> Self {
        Self {
            session_id: Uuid::new_v4(),
            player_id: None,
            token,
            engine,
            config,
            last_window: Instant::now(),
            msg_count: 0,
            abuse_strikes: 0,
        }
    }

    fn allow_message(&mut self) -> bool {
        if self.last_window.elapsed() >= Duration::from_secs(1) {
            self.last_window = Instant::now();
            self.msg_count = 0;
        }
        self.msg_count += 1;
        if self.msg_count > self.config.server.input_rate_limit_per_sec {
            self.abuse_strikes += 1;
            return false;
        }
        true
    }

    fn handle_client(&mut self, msg: ClientMessage, ctx: &mut ws::WebsocketContext<Self>) {
        let player_id = match self.player_id {
            Some(id) => id,
            None => return,
        };
        match msg {
            ClientMessage::Input(input) => {
                self.engine.do_send(InputMsg { player_id, input });
            }
            ClientMessage::Spawn(request) => {
                self.engine.do_send(SpawnMsg { player_id, request });
            }
            ClientMessage::Craft(request) => {
                self.engine.do_send(CraftMsg { player_id, request });
            }
            ClientMessage::Trade(request) => {
                self.engine.do_send(TradeMsg { player_id, request });
            }
            ClientMessage::NpcAction(request) => {
                self.engine.do_send(NpcActionMsg { player_id, request });
            }
            ClientMessage::AcceptQuest(request) => {
                self.engine.do_send(AcceptQuestMsg { player_id, request });
            }
            ClientMessage::Slot(request) => {
                self.engine.do_send(SlotMsg { player_id, request });
            }
            ClientMessage::SwapSlots(request) => {
                self.engine.do_send(SwapSlotsMsg { player_id, request });
            }
            ClientMessage::Name(request) => {
                self.engine.do_send(NameMsg { player_id, request });
            }
        }
        let _ = ctx;
    }

    fn send_json(ctx: &mut ws::WebsocketContext<Self>, message: ServerMessage) {
        if let Ok(text) = serde_json::to_string(&message) {
            ctx.text(text);
        }
    }
}

impl Actor for GameSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let join = Join {
            session_id: self.session_id,
            token: self.token,
            addr: ctx.address().recipient(),
        };
        let fut = self.engine.send(join).into_actor(self).map(|res, act, ctx| {
            if let Ok(result) = res {
                act.player_id = Some(result.player_id);
                act.token = Some(result.token);
                let welcome = ServerMessage::Welcome {
                    id: result.player_id.to_string(),
                    token: result.token.to_string(),
                    version: 2,
                    spawned: result.spawned,
                };
                GameSession::send_json(ctx, welcome);
            } else {
                ctx.stop();
            }
        });
        ctx.spawn(fut);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.engine.do_send(Leave {
            session_id: self.session_id,
        });
        Running::Stop
    }
}

impl Handler<ServerMessage> for GameSession {
    type Result = ();

    fn handle(&mut self, msg: ServerMessage, ctx: &mut Self::Context) -> Self::Result {
        let should_close = matches!(msg, ServerMessage::SessionRevoked { .. });
        GameSession::send_json(ctx, msg);
        if should_close {
            ctx.close(None);
            ctx.stop();
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for GameSession {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match item {
            Ok(msg) => msg,
            Err(_) => {
                ctx.stop();
                return;
            }
        };

        match msg {
            ws::Message::Text(text) => {
                if text.len() > self.config.server.max_message_bytes {
                    ctx.stop();
                    return;
                }
                if !self.allow_message() {
                    if self.abuse_strikes > 3 {
                        ctx.stop();
                    }
                    return;
                }
                match serde_json::from_str::<ClientMessage>(&text) {
                    Ok(message) => self.handle_client(message, ctx),
                    Err(_) => {
                        self.abuse_strikes += 1;
                        if self.abuse_strikes > 3 {
                            ctx.stop();
                        }
                    }
                }
            }
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Pong(_) => {}
            ws::Message::Close(_) => ctx.stop(),
            _ => {}
        }
    }
}
