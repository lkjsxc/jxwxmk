use actix::prelude::*;
use actix_web_actors::ws;
use game::{GameEngine, GameEvent, ClientMessage as GameClientMessage, OutboundMessage};
use protocol::{ClientMessage, ServerMessage, ErrorData};
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use std::time::{Instant, Duration};
use log::{info, error, warn};

pub struct WsSession {
    pub game_addr: Addr<GameEngine>,
    pub db_pool: Pool<Postgres>,
    pub token: Uuid,
    pub player_id: Uuid,
    pub last_heartbeat: Instant,
    
    // Rate Limiting
    pub msg_count: u32,
    pub last_msg_reset: Instant,
    pub max_msg_per_sec: u32,
    pub max_msg_bytes: usize,
}

impl WsSession {
    pub fn new(game_addr: Addr<GameEngine>, db_pool: Pool<Postgres>, token: Uuid) -> Self {
        Self {
            game_addr,
            db_pool,
            token,
            player_id: Uuid::nil(),
            last_heartbeat: Instant::now(),
            msg_count: 0,
            last_msg_reset: Instant::now(),
            max_msg_per_sec: 100, // Increased for stability
            max_msg_bytes: 65536, // Increased for large deltas
        }
    }

    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(Duration::from_secs(5), |act, ctx| {
            if Instant::now().duration_since(act.last_heartbeat) > Duration::from_secs(30) {
                warn!("WS Heartbeat failed for player {}, disconnecting!", act.player_id);
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }

    fn check_rate_limit(&mut self, bytes_len: usize) -> bool {
        if bytes_len > self.max_msg_bytes {
            warn!("Message too large: {} bytes", bytes_len);
            return false;
        }

        let now = Instant::now();
        if now.duration_since(self.last_msg_reset) > Duration::from_secs(1) {
            self.msg_count = 1;
            self.last_msg_reset = now;
            true
        } else {
            if self.msg_count < self.max_msg_per_sec {
                self.msg_count += 1;
                true
            } else {
                warn!("Rate limit exceeded for player {}", self.player_id);
                false
            }
        }
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("WsSession started for token: {}", self.token);
        self.hb(ctx);
        
        let token = self.token;
        let pool = self.db_pool.clone();
        let game_addr = self.game_addr.clone();
        let recipient = ctx.address().recipient();

        let fut = async move {
            persistence::player::load_player_by_token(&pool, token).await
        }
        .into_actor(self)
        .then(move |res, act, ctx| {
            match res {
                Ok(Some(player)) => {
                    info!("Player found in DB: {} (id: {})", player.name, player.id);
                    act.player_id = player.id;
                    game_addr.do_send(GameClientMessage(GameEvent::PlayerRejoin {
                        state: player,
                        recipient,
                    }));
                },
                Ok(None) => {
                    warn!("Token not found in DB: {}", token);
                    let msg = ServerMessage::Error {
                        data: ErrorData {
                            code: "invalid_token".into(),
                            message: "Invalid or expired session token".into(),
                            details: None,
                        }
                    };
                    if let Ok(json) = serde_json::to_string(&msg) {
                        ctx.text(json);
                    }
                    ctx.stop();
                },
                Err(e) => {
                    error!("DB Error loading player for token {}: {:?}", token, e);
                    ctx.stop();
                }
            }
            fut::ready(())
        });

        ctx.wait(fut);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        info!("WsSession stopping for player: {}", self.player_id);
        if self.player_id != Uuid::nil() {
            self.game_addr.do_send(GameClientMessage(GameEvent::PlayerLeave {
                player_id: self.player_id,
            }));
        }
        Running::Stop
    }
}

impl Handler<OutboundMessage> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: OutboundMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(e) => {
                error!("WS Protocol Error: {:?}", e);
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            ws::Message::Ping(bytes) => {
                self.last_heartbeat = Instant::now();
                ctx.pong(&bytes);
            }
            ws::Message::Pong(_) => {
                self.last_heartbeat = Instant::now();
            }
            ws::Message::Text(text) => {
                if !self.check_rate_limit(text.len()) {
                    let err = ServerMessage::Error {
                        data: ErrorData {
                            code: "rate_limited".into(),
                            message: "Too many messages or message too large".into(),
                            details: None,
                        }
                    };
                    if let Ok(json) = serde_json::to_string(&err) {
                        ctx.text(json);
                    }
                    ctx.stop();
                    return;
                }

                if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                    match client_msg {
                        ClientMessage::Input(data) => {
                            self.game_addr.do_send(GameClientMessage(GameEvent::Input {
                                player_id: self.player_id,
                                dx: data.dx, dy: data.dy,
                                attack: data.attack, interact: data.interact,
                                aim: data.aim.map(|a| world::Vec2 { x: a.x, y: a.y }),
                            }));
                        }
                        ClientMessage::Spawn(data) => {
                            self.game_addr.do_send(GameClientMessage(GameEvent::Spawn {
                                player_id: self.player_id,
                                settlement_id: data.settlement_id,
                            }));
                        }
                        ClientMessage::Craft(data) => {
                            self.game_addr.do_send(GameClientMessage(GameEvent::Craft {
                                player_id: self.player_id,
                                recipe_id: data.recipe,
                            }));
                        }
                        ClientMessage::AcceptQuest(data) => {
                            self.game_addr.do_send(GameClientMessage(GameEvent::AcceptQuest {
                                player_id: self.player_id,
                                quest_id: data.quest_id,
                            }));
                        },
                        ClientMessage::Slot(data) => {
                            self.game_addr.do_send(GameClientMessage(GameEvent::Slot {
                                player_id: self.player_id,
                                slot: data.slot,
                            }));
                        },
                        ClientMessage::SwapSlots(data) => {
                            self.game_addr.do_send(GameClientMessage(GameEvent::SwapSlots {
                                player_id: self.player_id,
                                from: data.from, to: data.to,
                            }));
                        },
                        _ => {}
                    }
                } else {
                    warn!("Invalid protocol message from player {}: {}", self.player_id, text);
                }
            }
            ws::Message::Binary(_) => (),
            ws::Message::Close(reason) => {
                info!("WS Close received for player {}: {:?}", self.player_id, reason);
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => (),
            ws::Message::Nop => (),
        }
    }
}
