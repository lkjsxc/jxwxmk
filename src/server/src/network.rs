use actix_web::{web, HttpResponse, Responder, HttpRequest};
use actix_web_actors::ws;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, debug, error};
use crate::{AppState, simulation::GameEvent};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(health_check))
            .route("/ws", web::get().to(websocket_handler)),
    );
}

async fn health_check() -> impl Responder {
    info!("Health check requested");
    HttpResponse::Ok().json("{"status": "healthy"}")
}

async fn websocket_handler(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    info!("WebSocket connection attempt");
    
    ws::start(
        WebSocketSession {
            id: 0,
            simulation: data.simulation.clone(),
            db_pool: data.db_pool.clone(),
        },
        &req,
        stream,
    )
}

pub struct WebSocketSession {
    pub id: u64,
    pub simulation: Arc<Mutex<crate::simulation::GameSimulation>>,
    pub db_pool: sqlx::PgPool,
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                debug!("Received Ping: {:?}", msg);
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                debug!("Received Pong");
            }
            Ok(ws::Message::Text(text)) => {
                debug!("Received text message: {}", text);
                self.handle_text_message(text, ctx);
            }
            Ok(ws::Message::Binary(bin)) => {
                debug!("Received binary message: {} bytes", bin.len());
                self.handle_binary_message(bin, ctx);
            }
            Ok(ws::Message::Close(reason)) => {
                info!("Client disconnected: {:?}", reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                debug!("Received continuation frame");
                ctx.stop();
            }
            Ok(ws::Message::Nop) => {
                debug!("Received Nop");
            }
            Err(err) => {
                error!("WebSocket protocol error: {}", err);
                ctx.stop();
            }
        }
    }
}

impl WebSocketSession {
    fn handle_text_message(&self, text: String, ctx: &mut ws::WebsocketContext<Self>) {
        // Parse JSON message
        match serde_json::from_str::<ClientMessage>(&text) {
            Ok(message) => {
                self.process_client_message(message, ctx);
            }
            Err(e) => {
                error!("Failed to parse client message: {}", e);
                ctx.text("{"error": "Invalid message format"}");
            }
        }
    }
    
    fn handle_binary_message(&self, bin: Vec<u8>, ctx: &mut ws::WebsocketContext<Self>) {
        // Parse binary protocol message
        match crate::protocol::parse_binary_message(&bin) {
            Ok(message) => {
                self.process_client_message(message, ctx);
            }
            Err(e) => {
                error!("Failed to parse binary message: {}", e);
                let error_msg = serde_json::to_string(&ServerMessage::Error {
                    message: "Invalid binary message".to_string()
                }).unwrap();
                ctx.text(error_msg);
            }
        }
    }
    
    fn process_client_message(&self, message: ClientMessage, ctx: &mut ws::WebsocketContext<Self>) {
        match message {
            ClientMessage::Authenticate { token } => {
                self.handle_authentication(token, ctx);
            }
            ClientMessage::Input { input, sequence } => {
                self.handle_player_input(input, sequence, ctx);
            }
            ClientMessage::Ping => {
                self.handle_ping(ctx);
            }
        }
    }
    
    fn handle_authentication(&self, token: String, ctx: &mut ws::WebsocketContext<Self>) {
        info!("Client authentication attempt");
        // Validate JWT token
        // On success, send authentication response
        let response = ServerMessage::Authenticated {
            player_id: "player_123".to_string(),
            server_tick: self.simulation.lock().await.current_tick(),
        };
        
        let json_response = serde_json::to_string(&response).unwrap();
        ctx.text(json_response);
    }
    
    fn handle_player_input(&self, input: PlayerInput, sequence: u32, ctx: &mut ws::WebsocketContext<Self>) {
        debug!("Received player input: {:?}", input);
        
        // Send input to simulation
        let event = GameEvent::PlayerInput {
            player_id: "player_123".to_string(),
            input: input.into(),
        };
        
        if let Err(e) = self.simulation.lock().await.event_sender.send(event).await {
            error!("Failed to send player input to simulation: {}", e);
        }
    }
    
    fn handle_ping(&self, ctx: &mut ws::WebsocketContext<Self>) {
        debug!("Received ping, sending pong");
        let response = ServerMessage::Pong {
            server_tick: self.simulation.lock().await.current_tick(),
        };
        
        let json_response = serde_json::to_string(&response).unwrap();
        ctx.text(json_response);
    }
}

#[derive(Debug, serde::Deserialize)]
pub enum ClientMessage {
    Authenticate { token: String },
    Input { input: PlayerInput, sequence: u32 },
    Ping,
}

#[derive(Debug, serde::Serialize)]
pub enum ServerMessage {
    Authenticated { player_id: String, server_tick: u64 },
    StateUpdate { tick: u64, entities: Vec<EntityState> },
    Error { message: String },
    Pong { server_tick: u64 },
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PlayerInput {
    pub movement: MovementInput,
    pub actions: Vec<PlayerAction>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct MovementInput {
    pub direction: (f32, f32),
    pub speed: f32,
    pub sprinting: bool,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum PlayerAction {
    Attack,
    UseItem { slot: usize },
    Craft { recipe_id: String },
    Interact,
}

#[derive(Debug, serde::Serialize)]
pub struct EntityState {
    pub id: String,
    pub position: (f32, f32),
    pub velocity: (f32, f32),
    pub health: f32,
    pub max_health: f32,
    pub entity_type: String,
}

impl From<PlayerInput> for crate::simulation::PlayerInput {
    fn from(input: PlayerInput) -> Self {
        Self {
            movement: crate::simulation::MovementInput {
                direction: input.movement.direction,
                speed: input.movement.speed,
                sprinting: input.movement.sprinting,
            },
            actions: input.actions.into_iter().map(|a| a.into()).collect(),
            sequence: 0, // Will be set by network layer
        }
    }
}

impl From<PlayerAction> for crate::simulation::PlayerAction {
    fn from(action: PlayerAction) -> Self {
        match action {
            PlayerAction::Attack => Self::Attack,
            PlayerAction::UseItem { slot } => Self::UseItem { slot },
            PlayerAction::Craft { recipe_id } => Self::Craft { recipe_id },
            PlayerAction::Interact => Self::Interact,
        }
    }
}