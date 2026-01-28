use std::collections::HashMap;
use tokio::sync::mpsc;
use protocol::{ClientMessage, ServerMessage, EntitySnapshot, Vec2};
use world::{World, Entity};
use config::Config;
use uuid::Uuid;


pub enum EngineEvent {
    PlayerJoined(Uuid, mpsc::UnboundedSender<ServerMessage>),
    PlayerLeft(Uuid),
    ClientMsg(Uuid, ClientMessage),
}

pub struct GameEngine {
    world: World,
    config: Config,
    event_rx: mpsc::UnboundedReceiver<EngineEvent>,
    players: HashMap<Uuid, mpsc::UnboundedSender<ServerMessage>>,
}

impl GameEngine {
    pub fn new(config: Config) -> (Self, mpsc::UnboundedSender<EngineEvent>) {
        let (tx, rx) = mpsc::unbounded_channel();
        (
            Self {
                world: World::new(),
                config,
                event_rx: rx,
                players: HashMap::new(),
            },
            tx,
        )
    }

    pub async fn run(mut self) {
        let tick_rate = self.config.server.tick_rate;
        let tick_duration = std::time::Duration::from_secs_f64(1.0 / tick_rate as f64);
        let mut interval = tokio::time::interval(tick_duration);

        loop {
            interval.tick().await;
            self.tick();
        }
    }

    fn tick(&mut self) {
        let mut events = Vec::new();
        while let Ok(event) = self.event_rx.try_recv() {
            events.push(event);
        }

        for event in events {
            match event {
                EngineEvent::PlayerJoined(id, tx) => {
                    self.players.insert(id, tx.clone());
                    
                    // Create player entity
                    let entity_id = format!("player-{}", id);
                    let entity = Entity {
                        id: entity_id.clone(),
                        kind: "player".to_string(),
                        subtype: "human".to_string(),
                        pos: Vec2 { x: 0.0, y: 0.0 },
                        hp: Some(100.0),
                        max_hp: Some(100.0),
                        level: Some(1),
                        name: Some(format!("Player {}", id)),
                    };
                    
                    self.world.get_chunk_mut((0, 0)).entities.insert(entity_id.clone(), entity);
                    self.world.players.insert(id, entity_id);

                    let _ = tx.send(ServerMessage::Welcome {
                        id,
                        token: Uuid::new_v4(),
                        version: 3,
                        spawned: true,
                    });
                }
                EngineEvent::PlayerLeft(id) => {
                    if let Some(entity_id) = self.world.players.remove(&id) {
                        self.world.get_chunk_mut((0, 0)).entities.remove(&entity_id);
                    }
                    self.players.remove(&id);
                }
                EngineEvent::ClientMsg(id, msg) => {
                    if let ClientMessage::Input(input) = msg {
                        if let Some(entity_id) = self.world.players.get(&id).cloned() {
                            if let Some(entity) = self.world.get_chunk_mut((0, 0)).entities.get_mut(&entity_id) {
                                // Simple movement
                                let speed = 0.5;
                                entity.pos.x += input.dx * speed;
                                entity.pos.y += input.dy * speed;
                            }
                        }
                    }
                }

            }
        }

        // Broadcast deltas
        let updates: Vec<EntitySnapshot> = self.world.get_chunk_mut((0, 0))
            .entities.values().map(|e| e.to_snapshot()).collect();
            
        let msg = ServerMessage::EntityDelta {
            data: protocol::EntityDeltaData {
                chunk: (0, 0),
                updates,
                removes: Vec::new(),
            }
        };

        for tx in self.players.values() {
            let _ = tx.send(msg.clone());
        }
    }
}