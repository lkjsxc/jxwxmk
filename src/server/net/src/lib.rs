use actix::prelude::*;
use actix_web::{web, HttpResponse, Result, Error};
use actix_web_actors::ws;
use protocol::{ClientMessage, ServerMessage};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct SessionRegistry {
    sessions: Arc<RwLock<HashMap<Uuid, Addr<GameSession>>>>,
}

impl SessionRegistry {
    pub fn new() -> Self {
        SessionRegistry {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register(
        &self,
        player_id: Uuid,
        addr: Addr<GameSession>,
    ) {
        let mut sessions = self.sessions.write().await;
        sessions.insert(player_id, addr);
    }

    pub async fn unregister(
        &self,
        player_id: Uuid,
    ) {
        let mut sessions = self.sessions.write().await;
        sessions.remove(&player_id);
    }

    pub async fn revoke_session(
        &self,
        player_id: Uuid,
    ) {
        let sessions = self.sessions.read().await;
        if let Some(addr) = sessions.get(&player_id) {
            addr.do_send(SessionRevoked);
        }
    }

    pub async fn is_connected(&self,
        player_id: Uuid,
    ) -> bool {
        let sessions = self.sessions.read().await;
        sessions.contains_key(&player_id)
    }
}

pub struct GameSession {
    pub player_id: Uuid,
    pub token: Uuid,
    pub registry: Arc<SessionRegistry>,
    pub hb: Instant,
    pub spawned: bool,
}

impl Actor for GameSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb = Instant::now();
        
        let welcome = ServerMessage::Welcome {
            id: self.player_id,
            token: self.token,
            version: 3,
            spawned: self.spawned,
        };
        
        if let Ok(json) = serde_json::to_string(&welcome) {
            ctx.text(json);
        }
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        let registry = Arc::clone(&self.registry);
        let player_id = self.player_id;
        
        actix::spawn(async move {
            registry.unregister(player_id).await;
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for GameSession {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                self.handle_message(&text, ctx);
            }
            Ok(ws::Message::Binary(bin)) => {
                ctx.binary(bin);
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

impl GameSession {
    fn handle_message(
        &mut self,
        text: &str,
        ctx: &mut ws::WebsocketContext<Self>,
    ) {
        match serde_json::from_str::<ClientMessage>(text) {
            Ok(message) => {
                log::debug!("Received message from {}: {:?}", self.player_id, message);
                
                match message {
                    ClientMessage::Spawn(_) => {
                        self.spawned = true;
                        let response = ServerMessage::PlayerUpdate {
                            data: protocol::PlayerUpdateData {
                                id: self.player_id,
                                name: "Player".to_string(),
                                spawned: true,
                                vitals: protocol::Vitals {
                                    hp: 30.0,
                                    max_hp: 30.0,
                                    hunger: 80.0,
                                    max_hunger: 100.0,
                                    temperature: 50.0,
                                    max_temperature: 100.0,
                                },
                                inventory: vec![None; 30],
                                active_slot: 0,
                                level: 1,
                                xp: 0,
                                stats: protocol::PlayerStats {
                                    steps: 0,
                                    kills: 0,
                                    crafts: 0,
                                    gathers: 0,
                                    deaths: 0,
                                },
                                quests: vec![],
                                achievements: vec![],
                            },
                        };
                        if let Ok(json) = serde_json::to_string(&response) {
                            ctx.text(json);
                        }
                    }
                    ClientMessage::Input(data) => {
                        if self.spawned {
                            let notification = ServerMessage::Notification {
                                data: protocol::NotificationData {
                                    text: format!("Moving: dx={}, dy={}", data.dx, data.dy),
                                },
                            };
                            if let Ok(json) = serde_json::to_string(&notification) {
                                ctx.text(json);
                            }
                        }
                    }
                    ClientMessage::Craft(data) => {
                        let notification = ServerMessage::Notification {
                            data: protocol::NotificationData {
                                text: format!("Crafting: {}", data.recipe),
                            },
                        };
                        if let Ok(json) = serde_json::to_string(&notification) {
                            ctx.text(json);
                        }
                    }
                    ClientMessage::Attack(data) => {
                        // Simplified attack handling - would integrate with game engine
                        let combat_result = ServerMessage::CombatResult {
                            data: protocol::CombatResultData {
                                target_id: data.target_id,
                                damage: 10.0,
                                hit: true,
                                critical: false,
                            },
                        };
                        if let Ok(json) = serde_json::to_string(&combat_result) {
                            ctx.text(json);
                        }
                    }
                    ClientMessage::Gather(data) => {
                        // Simplified gather handling
                        let depleted = ServerMessage::ResourceDepleted {
                            data: protocol::ResourceDepletedData {
                                resource_id: data.resource_id,
                                items_received: vec![protocol::InventorySlot {
                                    item: "wood".to_string(),
                                    count: 1,
                                }],
                            },
                        };
                        if let Ok(json) = serde_json::to_string(&depleted) {
                            ctx.text(json);
                        }
                    }
                    _ => {
                        let notification = ServerMessage::Notification {
                            data: protocol::NotificationData {
                                text: "Message received".to_string(),
                            },
                        };
                        if let Ok(json) = serde_json::to_string(&notification) {
                            ctx.text(json);
                        }
                    }
                }
            }
            Err(e) => {
                log::warn!("Failed to parse message from {}: {}", self.player_id, e);
                let error = ServerMessage::Error {
                    data: protocol::ErrorData {
                        code: "invalid_message".to_string(),
                        message: "Invalid message format".to_string(),
                        details: None,
                    },
                };
                if let Ok(json) = serde_json::to_string(&error) {
                    ctx.text(json);
                }
            }
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SessionRevoked;

impl Handler<SessionRevoked> for GameSession {
    type Result = ();

    fn handle(&mut self,
        _msg: SessionRevoked,
        ctx: &mut Self::Context,
    ) {
        let revoked = ServerMessage::SessionRevoked {
            reason: "login_elsewhere".to_string(),
        };
        if let Ok(json) = serde_json::to_string(&revoked) {
            ctx.text(json);
        }
        ctx.stop();
    }
}

pub async fn ws_route(
    req: actix_web::HttpRequest,
    stream: web::Payload,
    registry: web::Data<Arc<SessionRegistry>>,
) -> Result<HttpResponse, Error> {
    let token = req
        .query_string()
        .split('&')
        .find_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            let key = parts.next()?;
            let value = parts.next()?;
            if key == "token" {
                Uuid::parse_str(value).ok()
            } else {
                None
            }
        });

    let (player_id, session_token, spawned) = if let Some(t) = token {
        (t, t, false)
    } else {
        let new_id = Uuid::new_v4();
        (new_id, new_id, false)
    };

    let session = GameSession {
        player_id,
        token: session_token,
        registry: Arc::clone(registry.get_ref()),
        hb: Instant::now(),
        spawned,
    };

    ws::start(session, &req, stream)
}
