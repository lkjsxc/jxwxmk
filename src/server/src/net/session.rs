use std::time::{Duration, Instant};

use actix::{Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, StreamHandler, WrapFuture};
use actix_web_actors::ws;
use uuid::Uuid;

use crate::config::Config;
use crate::game::{EngineEventCommand, GameEngine, JoinCommand, LeaveCommand};
use crate::protocol::{ClientMessage, ServerMessage};
use crate::net::session_map::map_message;

pub struct GameSession {
    pub player_id: Uuid,
    pub token: Uuid,
    pub engine: Addr<GameEngine>,
    pub config: Config,
    pub window_start: Instant,
    pub window_count: u32,
    pub invalid_count: u32,
}

impl GameSession {
    pub fn new(player_id: Uuid, token: Uuid, config: Config, engine: Addr<GameEngine>) -> Self {
        Self {
            player_id,
            token,
            engine,
            config,
            window_start: Instant::now(),
            window_count: 0,
            invalid_count: 0,
        }
    }

    fn allow_message(&mut self) -> bool {
        let now = Instant::now();
        if now.duration_since(self.window_start) >= Duration::from_secs(1) {
            self.window_start = now;
            self.window_count = 0;
        }
        if self.window_count >= self.config.server.input_rate_limit_per_sec {
            return false;
        }
        self.window_count += 1;
        true
    }

    fn bump_invalid(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        self.invalid_count += 1;
        if self.invalid_count > 5 {
            ctx.close(None);
            ctx.stop();
        }
    }
}

impl Actor for GameSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let session = ctx.address().recipient();
        let engine = self.engine.clone();
        let player_id = self.player_id;
        let token = self.token;
        ctx.spawn(
            async move { engine.send(JoinCommand { player_id, token, session }).await }
                .into_actor(self)
                .map(|res, act, ctx| {
                    match res {
                        Ok(join) => {
                            act.token = join.token;
                            let welcome = ServerMessage::Welcome {
                                id: join.id,
                                token: join.token,
                                version: 2,
                                spawned: join.spawned,
                            };
                            let _ = ctx.text(serde_json::to_string(&welcome).unwrap());
                        }
                        Err(_) => {
                            ctx.close(None);
                            ctx.stop();
                        }
                    }
                }),
        );
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::Running {
        self.engine.do_send(LeaveCommand { player_id: self.player_id });
        actix::Running::Stop
    }
}

impl actix::Handler<ServerMessage> for GameSession {
    type Result = ();

    fn handle(&mut self, msg: ServerMessage, ctx: &mut Self::Context) -> Self::Result {
        if let Ok(text) = serde_json::to_string(&msg) {
            ctx.text(text);
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for GameSession {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let Ok(msg) = item else {
            self.bump_invalid(ctx);
            return;
        };

        match msg {
            ws::Message::Text(text) => {
                if text.len() > self.config.server.max_message_bytes {
                    ctx.close(None);
                    ctx.stop();
                    return;
                }
                if !self.allow_message() {
                    ctx.close(None);
                    ctx.stop();
                    return;
                }
                let parsed: Result<ClientMessage, _> = serde_json::from_str(&text);
                match parsed {
                    Ok(message) => {
                        if let Some(event) = map_message(self, message) {
                            self.engine.do_send(EngineEventCommand { event });
                        } else {
                            self.bump_invalid(ctx);
                        }
                    }
                    Err(_) => self.bump_invalid(ctx),
                }
            }
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Close(_) => {
                ctx.close(None);
                ctx.stop();
            }
            _ => {}
        }
    }
}
