use actix_web::{web, HttpResponse, Responder, HttpRequest};
use actix_web_actors::ws;
use actix::{Actor, StreamHandler, ActorContext, AsyncContext, Message};
use std::sync::Arc;
use tokio::sync::{Mutex, broadcast};
use tracing::{info, debug, error};
use crate::{AppState, simulation::GameEvent};
use crate::protocol::{
    self, ClientMessage, ServerMessage, PlayerInput, PlayerAction, EntityState
};
use tokio_stream::wrappers::BroadcastStream;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(health_check))
            .route("/ws", web::get().to(websocket_handler)),
    );
}

async fn health_check() -> impl Responder {
    info!("Health check requested");
    HttpResponse::Ok().json(serde_json::json!({"status": "healthy"}))
}

async fn websocket_handler(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    info!("WebSocket connection attempt");
    
    let rx = data.broadcast_sender.subscribe();

    ws::start(
        WebSocketSession {
            id: 0,
            simulation: data.simulation.clone(),
            db_pool: data.db_pool.clone(),
            broadcast_receiver: Some(rx),
        },
        &req,
        stream,
    )
}

pub struct WebSocketSession {
    pub id: u64,
    pub simulation: Arc<Mutex<crate::simulation::GameSimulation>>,
    pub db_pool: sqlx::PgPool,
    pub broadcast_receiver: Option<broadcast::Receiver<ServerMessage>>,
}

#[derive(Message)]
#[rtype(result = "()")]
struct BroadcastMessage(ServerMessage);

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        if let Some(rx) = self.broadcast_receiver.take() {
            let stream = BroadcastStream::new(rx);
            ctx.add_stream(stream);
        }
    }
}

impl StreamHandler<Result<ServerMessage, tokio_stream::wrappers::errors::BroadcastStreamRecvError>> for WebSocketSession {
    fn handle(
        &mut self,
        msg: Result<ServerMessage, tokio_stream::wrappers::errors::BroadcastStreamRecvError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(server_msg) => {
                let json = serde_json::to_string(&server_msg).unwrap_or_default();
                ctx.text(json);
            }
            Err(e) => {
                error!("Broadcast receive error: {}", e);
            }
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
            }
             Ok(ws::Message::Text(text)) => {
                self.handle_text_message(text.to_string(), ctx);
            }
            Ok(ws::Message::Binary(bin)) => {
                self.handle_binary_message(bin.to_vec(), ctx);
            }
            Ok(ws::Message::Close(reason)) => {
                info!("Client disconnected: {:?}", reason);
                ctx.stop();
            }
            Err(err) => {
                error!("WebSocket protocol error: {}", err);
                ctx.stop();
            }
            _ => {}
        }
    }
}

impl WebSocketSession {
    fn handle_text_message(&self, text: String, ctx: &mut ws::WebsocketContext<Self>) {
        match serde_json::from_str::<ClientMessage>(&text) {
            Ok(message) => {
                self.process_client_message(message, ctx);
            }
            Err(e) => {
                error!("Failed to parse client message: {}", e);
                ctx.text(r#"{"error": "Invalid message format"}"#);
            }
        }
    }
    
    fn handle_binary_message(&self, bin: Vec<u8>, ctx: &mut ws::WebsocketContext<Self>) {
        match crate::protocol::parse_binary_message(&bin) {
            Ok(binary_message) => {
                let client_message = match binary_message.message_type {
                    crate::protocol::MessageType::Authenticate => {
                        match crate::protocol::deserialize_authenticate_message(&binary_message.payload) {
                            Ok(token) => ClientMessage::Authenticate { token },
                            Err(e) => {
                                error!("Failed to deserialize authenticate message: {}", e);
                                return;
                            }
                        }
                    }
                    crate::protocol::MessageType::Input => {
                        match crate::protocol::deserialize_input_message(&binary_message.payload) {
                            Ok((protocol_input, _)) => {
                                ClientMessage::Input { input: protocol_input, sequence: binary_message.sequence }
                            },
                            Err(e) => {
                                error!("Failed to deserialize input message: {}", e);
                                return;
                            }
                        }
                    }
                    crate::protocol::MessageType::Ping => ClientMessage::Ping,
                    _ => {
                        error!("Unsupported binary message type: {:?}", binary_message.message_type);
                        return;
                    }
                };
                self.process_client_message(client_message, ctx);
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
            ClientMessage::Craft { recipe_id } => {
                self.handle_craft(recipe_id, ctx);
            }
            ClientMessage::Ping => {
                self.handle_ping(ctx);
            }
        }
    }
    
    fn handle_authentication(&self, _token: String, ctx: &mut ws::WebsocketContext<Self>) {
        info!("Client authentication attempt");
        
        let response = ServerMessage::Authenticated {
            player_id: "player_123".to_string(), 
            server_tick: 0,
        };
        ctx.text(serde_json::to_string(&response).unwrap());
        
        // Send recipes
        let recipes = ServerMessage::Recipes {
            recipes: crate::systems::CraftingSystem::get_default_recipes().into_iter().map(|r| {
                crate::protocol::CraftingRecipe {
                    id: r.id,
                    name: r.name,
                    requirements: r.requirements,
                    result: crate::protocol::CraftingResult {
                        item_type: r.result.item_type,
                        quantity: r.result.quantity,
                    },
                    crafting_time: r.crafting_time,
                    tier: r.tier,
                }
            }).collect(),
        };
        ctx.text(serde_json::to_string(&recipes).unwrap());
        
        let event = GameEvent::PlayerConnected {
             player_id: "player_123".to_string(),
        };
        
        let sim = self.simulation.clone();
        ctx.spawn(actix::fut::wrap_future(async move {
            let sim = sim.lock().await;
            sim.event_sender.send(event).await.ok();
        }));
    }
    
    fn handle_player_input(&self, input: PlayerInput, sequence: u32, ctx: &mut ws::WebsocketContext<Self>) {
        let event = GameEvent::PlayerInput {
            player_id: "player_123".to_string(),
            input: crate::simulation::PlayerInput {
                movement: crate::simulation::MovementInput {
                    direction: input.movement.direction,
                    speed: input.movement.speed,
                    sprinting: input.movement.sprinting,
                },
                actions: input.actions.into_iter().map(|a| {
                    match a {
                        protocol::PlayerAction::Attack => crate::simulation::PlayerAction::Attack,
                        protocol::PlayerAction::UseItem { slot } => crate::simulation::PlayerAction::UseItem { slot },
                        protocol::PlayerAction::Craft { recipe_id } => crate::simulation::PlayerAction::Craft { recipe_id },
                        protocol::PlayerAction::Interact => crate::simulation::PlayerAction::Interact,
                    }
                }).collect(),
                sequence,
            },
        };
        
        let sim = self.simulation.clone();
        ctx.spawn(actix::fut::wrap_future(async move {
            let sim = sim.lock().await;
            sim.event_sender.send(event).await.ok();
        }));
    }

    fn handle_craft(&self, recipe_id: String, ctx: &mut ws::WebsocketContext<Self>) {
        let event = GameEvent::PlayerInput {
            player_id: "player_123".to_string(),
            input: crate::simulation::PlayerInput {
                movement: crate::simulation::MovementInput::default(),
                actions: vec![crate::simulation::PlayerAction::Craft { recipe_id }],
                sequence: 0,
            },
        };
        
        let sim = self.simulation.clone();
        ctx.spawn(actix::fut::wrap_future(async move {
            let sim = sim.lock().await;
            sim.event_sender.send(event).await.ok();
        }));
    }
    
    fn handle_ping(&self, ctx: &mut ws::WebsocketContext<Self>) {
        let response = ServerMessage::Pong {
            server_tick: 0,
        };
        ctx.text(serde_json::to_string(&response).unwrap());
    }
}