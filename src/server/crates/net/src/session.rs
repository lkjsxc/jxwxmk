use std::sync::Arc;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tokio::sync::{RwLock, mpsc};
use tokio::time::interval;
use uuid::Uuid;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_ws::{Message, Session as WsSession};
use futures::StreamExt;

use game::{GameHandle, GameEvent, GameResponse};
use persistence::PersistenceHandle;
use protocol::*;

#[derive(Clone, Copy)]
pub struct WsSessionConfig {
    pub heartbeat_interval: Duration,
    pub client_timeout: Duration,
}

pub type SessionsMap = Arc<RwLock<HashMap<Uuid, mpsc::UnboundedSender<ServerMessage>>>>;

pub struct GameSession {
    game: GameHandle,
    persistence: PersistenceHandle,
    sessions: SessionsMap,
    ws_config: WsSessionConfig,
    player_id: Option<Uuid>,
    token: Option<Uuid>,
    hb: Instant,
    last_input: Instant,
    input_count: u32,
}

impl GameSession {
    pub fn new(
        game: GameHandle,
        persistence: PersistenceHandle,
        sessions: SessionsMap,
        token: Option<Uuid>,
        ws_config: WsSessionConfig,
    ) -> Self {
        Self {
            game,
            persistence,
            sessions,
            ws_config,
            player_id: None,
            token,
            hb: Instant::now(),
            last_input: Instant::now(),
            input_count: 0,
        }
    }

    pub async fn run(mut self, mut session: WsSession, mut msg_stream: actix_ws::MessageStream) {
        let (tx, mut rx) = mpsc::unbounded_channel::<ServerMessage>();
        
        // Handle connection
        let player_id = self.handle_connect(tx.clone()).await;
        self.player_id = player_id;
        
        if player_id.is_none() {
            let _ = session.text(serde_json::to_string(&ServerMessage::Error {
                data: ErrorData {
                    code: "connection_failed".to_string(),
                    message: "Failed to establish connection".to_string(),
                    details: None,
                }
            }).unwrap_or_default()).await;
            let _ = session.close(None).await;
            return;
        }
        
        let player_id = player_id.unwrap();
        
        // Insert session into map
        {
            let mut sessions = self.sessions.write().await;
            sessions.insert(player_id, tx);
        }
        
        // Start heartbeat
        let mut heartbeat = interval(self.ws_config.heartbeat_interval);
        
        loop {
            tokio::select! {
                // Handle WebSocket messages
                Some(Ok(msg)) = msg_stream.next() => {
                    match msg {
                        Message::Ping(bytes) => {
                            self.hb = Instant::now();
                            let _ = session.pong(&bytes).await;
                        }
                        Message::Pong(_) => {
                            self.hb = Instant::now();
                        }
                        Message::Text(text) => {
                            self.hb = Instant::now(); // Update heartbeat on any message
                            self.handle_message(&text, player_id).await;
                        }
                        Message::Close(reason) => {
                            let _ = session.close(reason).await;
                            break;
                        }
                        _ => {}
                    }
                    
                    // Check heartbeat timeout
                    if Instant::now().duration_since(self.hb) > self.ws_config.client_timeout {
                        log::debug!("Client timeout, disconnecting");
                        let _ = session.close(None).await;
                        break;
                    }
                }
                
                // Handle outgoing messages
                Some(msg) = rx.recv() => {
                    let text = serde_json::to_string(&msg).unwrap_or_default();
                    if session.text(text).await.is_err() {
                        break;
                    }
                    if matches!(msg, ServerMessage::SessionRevoked { .. }) {
                        break;
                    }
                }
                
                // Heartbeat check
                _ = heartbeat.tick() => {
                    if Instant::now().duration_since(self.hb) > self.ws_config.client_timeout {
                        log::debug!("Client timeout, disconnecting");
                        let _ = session.close(None).await;
                        break;
                    }
                }
            }
        }
        
        // Cleanup
        self.handle_disconnect(player_id).await;
    }
    
    async fn handle_connect(&self, tx: mpsc::UnboundedSender<ServerMessage>) -> Option<Uuid> {
        let token = self.token;
        let persistence = self.persistence.clone();
        let game = self.game.clone();
        
        let player_id = if let Some(tok) = token {
            // Validate token
            // For now, generate new player
            Uuid::new_v4()
        } else {
            // New player
            Uuid::new_v4()
        };
        
        log::info!("New connection, player_id: {}", player_id);
        
        // Send welcome message
        let new_token = match persistence.rotate_token(player_id).await {
            Ok(token) => token,
            Err(e) => {
                log::error!("Failed to rotate token: {}", e);
                Uuid::new_v4()
            }
        };
        
        let welcome = ServerMessage::Welcome {
            id: player_id,
            token: new_token,
            version: PROTOCOL_VERSION,
            spawned: false,
        };
        
        log::info!("Sending welcome message to player {}", player_id);
        
        if let Err(e) = tx.send(welcome) {
            log::error!("Failed to send welcome message: {:?}", e);
            return None;
        }
        
        // Add player to game
        let _ = game.enqueue(GameEvent::Join { 
            player_id, 
            name: format!("Player {}", &player_id.to_string()[..8]) 
        }).await;
        
        Some(player_id)
    }
    
    async fn handle_disconnect(&self, player_id: Uuid) {
        // Remove from sessions
        let mut sessions = self.sessions.write().await;
        sessions.remove(&player_id);
        drop(sessions);
        
        // Notify game
        let _ = self.game.enqueue(GameEvent::Leave { player_id }).await;
    }
    
    async fn handle_message(&mut self, text: &str, player_id: Uuid) {
        // Rate limiting
        if self.input_count > 30 {
            let since = Instant::now().duration_since(self.last_input);
            if since < Duration::from_secs(1) {
                // Rate limited - ignore
                return;
            }
            self.input_count = 0;
            self.last_input = Instant::now();
        }
        self.input_count += 1;
        
        // Parse message
        match serde_json::from_str::<ClientMessage>(text) {
            Ok(msg) => {
                let event = match msg {
                    ClientMessage::Input(data) => GameEvent::Input { player_id, data },
                    ClientMessage::Spawn(data) => GameEvent::Spawn { player_id, data },
                    ClientMessage::Craft(data) => GameEvent::Craft { player_id, data },
                    ClientMessage::Trade(data) => GameEvent::Trade { player_id, data },
                    ClientMessage::NpcAction(data) => GameEvent::NpcAction { player_id, data },
                    ClientMessage::AcceptQuest(data) => GameEvent::AcceptQuest { player_id, data },
                    ClientMessage::Slot(data) => GameEvent::Slot { player_id, data },
                    ClientMessage::SwapSlots(data) => GameEvent::SwapSlots { player_id, data },
                    ClientMessage::Name(data) => GameEvent::Name { player_id, data },
                };
                
                let _ = self.game.enqueue(event).await;
            }
            Err(e) => {
                log::warn!("Failed to parse message: {}", e);
            }
        }
    }
}

pub async fn ws_handler(
    req: HttpRequest,
    body: web::Payload,
    game: web::Data<GameHandle>,
    persistence: web::Data<PersistenceHandle>,
    sessions: web::Data<SessionsMap>,
    ws_config: web::Data<WsSessionConfig>,
) -> Result<HttpResponse, actix_web::Error> {
    let token: Option<Uuid> = req.uri()
        .query()
        .and_then(|q| {
            q.split('&')
                .find(|p| p.starts_with("token="))
                .and_then(|p| p.split('=').nth(1))
                .and_then(|t| Uuid::parse_str(t).ok())
        });
    
    let (response, session, msg_stream) = actix_ws::handle(&req, body)?;
    
    let game = game.get_ref().clone();
    let persistence = persistence.get_ref().clone();
    let sessions = sessions.get_ref().clone();
    let ws_config = *ws_config.get_ref();
    
    let game_session = GameSession::new(game, persistence, sessions, token, ws_config);
    
    actix_web::rt::spawn(async move {
        game_session.run(session, msg_stream).await;
    });
    
    Ok(response)
}
