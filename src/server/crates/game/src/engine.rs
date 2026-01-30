use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use config::GameConfig;
use persistence::PersistenceHandle;
use protocol::*;
use systems::*;
use world::{PlayerState, World, generate_initial_settlement};

use crate::events::*;

const MAX_INPUT_QUEUE: usize = 1000;

pub struct GameEngine {
    world: World,
    config: Arc<GameConfig>,
    input_queue: VecDeque<GameEvent>,
    responses: Vec<GameResponse>,
    spawning_system: SpawningSystem,
    // Track which chunks each player has seen
    player_loaded_chunks: HashMap<Uuid, HashSet<(i32, i32)>>,
}

impl GameEngine {
    pub fn new(config: Arc<GameConfig>) -> Self {
        let mut world = World::new(&config.world);
        
        // Generate initial settlement
        let settlement = generate_initial_settlement();
        world.settlements.push(settlement);

        Self {
            world,
            config,
            input_queue: VecDeque::new(),
            responses: Vec::new(),
            spawning_system: SpawningSystem::new(),
            player_loaded_chunks: HashMap::new(),
        }
    }

    pub fn enqueue_event(&mut self, event: GameEvent) -> Result<(), GameEvent> {
        if self.input_queue.len() >= MAX_INPUT_QUEUE {
            return Err(event);
        }
        self.input_queue.push_back(event);
        Ok(())
    }

    pub fn drain_responses(&mut self) -> Vec<GameResponse> {
        std::mem::take(&mut self.responses)
    }

    pub fn tick(&mut self, dt: f64) {
        self.responses.clear();

        // 1. Process input events
        self.process_inputs();

        // 2. Activate/deactivate chunks
        let view_radius = self.config.world.view_radius_chunks;
        let sim_radius = self.config.world.sim_radius_chunks;
        
        for player_id in self.world.players.keys().copied().collect::<Vec<_>>() {
            self.world.update_interest_set(player_id, view_radius);
        }
        self.world.activate_chunks(view_radius, sim_radius);

        // 3. Run systems
        self.run_systems(dt);

        // 4. Build and send deltas (including chunks)
        self.build_deltas();
    }

    fn process_inputs(&mut self) {
        while let Some(event) = self.input_queue.pop_front() {
            match event {
                GameEvent::Join { player_id, name } => {
                    self.world.add_player(player_id, name);
                    // Initialize empty loaded chunks set for new player
                    self.player_loaded_chunks.insert(player_id, HashSet::new());
                }
                GameEvent::Leave { player_id } => {
                    self.world.remove_player(player_id);
                    self.player_loaded_chunks.remove(&player_id);
                }
                GameEvent::Input { player_id, data } => {
                    if let Err(e) = validate_input(&data, self.config.balance.player.interaction_range_wu) {
                        self.responses.push(GameResponse::Error {
                            player_id,
                            error: e.to_error_message(),
                        });
                    } else {
                        // Process input with split borrows to satisfy borrow checker
                        Self::process_player_input(&mut self.world, player_id, &data, &self.config, &mut self.responses);
                    }
                }
                GameEvent::Spawn { player_id, data } => {
                    if let Some(player) = self.world.players.get_mut(&player_id) {
                        // Spawn at settlement
                        let settlement = data.settlement_id
                            .and_then(|id| self.world.settlements.iter().find(|s| s.id == id))
                            .or_else(|| self.world.settlements.first());
                        
                        if let Some(settlement) = settlement {
                            player.spawn(settlement.x, settlement.y, Some(settlement.id));
                            
                            self.responses.push(GameResponse::PlayerUpdate {
                                player_id,
                                message: ServerMessage::PlayerUpdate {
                                    data: player.to_update_data(),
                                },
                            });
                        }
                    }
                }
                GameEvent::Craft { player_id, data } => {
                    if let Some(player) = self.world.players.get_mut(&player_id) {
                        if CraftingSystem::try_craft(player, &data.recipe, &self.config.crafting) {
                            self.responses.push(GameResponse::PlayerUpdate {
                                player_id,
                                message: ServerMessage::PlayerUpdate {
                                    data: player.to_update_data(),
                                },
                            });
                        } else {
                            self.responses.push(GameResponse::Error {
                                player_id,
                                error: ErrorData {
                                    code: "insufficient_items".to_string(),
                                    message: "Cannot craft: missing materials".to_string(),
                                    details: None,
                                },
                            });
                        }
                    }
                }
                GameEvent::Slot { player_id, data } => {
                    if let Some(player) = self.world.players.get_mut(&player_id) {
                        InteractionSystem::handle_slot_change(
                            player,
                            data.slot,
                            self.config.balance.player.hotbar_slots,
                        );
                    }
                }
                GameEvent::SwapSlots { player_id, data } => {
                    if let Some(player) = self.world.players.get_mut(&player_id) {
                        InteractionSystem::handle_swap_slots(
                            player,
                            data.from,
                            data.to,
                            self.config.balance.player.inventory_slots,
                        );
                    }
                }
                GameEvent::Name { player_id, data } => {
                    if let Some(player) = self.world.players.get_mut(&player_id) {
                        if validate_name(&data.name, self.config.server.limits.max_name_len).is_ok() {
                            player.name = data.name;
                        }
                    }
                }
                _ => {
                    // Other events not yet implemented
                }
            }
        }
    }

    fn run_systems(&mut self, dt: f64) {
        // Phase 1: Spawning system (needs &mut World)
        self.spawning_system.tick(&mut self.world, &self.config.spawning, dt);

        // Phase 2: Survival system (needs &mut [&mut PlayerState])
        {
            let mut players_vec: Vec<&mut PlayerState> = self.world.players.values_mut().collect();
            SurvivalSystem::tick(&mut players_vec, &self.config.survival, &self.config.balance, dt);
        }

        // Phase 3: Death and respawn
        // Extract settlement info first to avoid borrow issues
        let first_settlement = self.world.settlements.first()
            .map(|s| (s.x, s.y, s.id));
        
        let death_notifications = {
            let mut players_vec: Vec<&mut PlayerState> = self.world.players.values_mut().collect();
            DeathSystem::check_deaths(
                &mut players_vec,
                first_settlement,
                &self.config.balance,
                &self.config.settlements,
            )
        };

        for (player_id, notification) in death_notifications {
            self.responses.push(GameResponse::ToPlayer {
                player_id,
                message: ServerMessage::Notification { data: notification },
            });
        }

        // Phase 4: Auto-respawn
        let ready_to_respawn = {
            let mut players_vec: Vec<&mut PlayerState> = self.world.players.values_mut().collect();
            DeathSystem::check_respawns(&mut players_vec, dt)
        };
        
        for player_id in ready_to_respawn {
            if let Some(settlement) = self.world.settlements.first() {
                let settlement_pos = (settlement.x, settlement.y);
                if let Some(player) = self.world.players.get_mut(&player_id) {
                    player.spawned = true;
                    player.respawn_cooldown = 0.0;
                    player.x = settlement_pos.0;
                    player.y = settlement_pos.1;
                    
                    self.responses.push(GameResponse::PlayerUpdate {
                        player_id,
                        message: ServerMessage::PlayerUpdate {
                            data: player.to_update_data(),
                        },
                    });
                }
            }
        }

        // Phase 5: Achievements
        let achievements = {
            let mut players_vec: Vec<&mut PlayerState> = self.world.players.values_mut().collect();
            AchievementSystem::check_achievements(&mut players_vec, &self.config.achievements)
        };
        
        for (player_id, achievement, notification) in achievements {
            self.responses.push(GameResponse::ToPlayer {
                player_id,
                message: ServerMessage::Achievement { data: achievement },
            });
            self.responses.push(GameResponse::ToPlayer {
                player_id,
                message: ServerMessage::Notification { data: notification },
            });
        }
    }

    fn build_deltas(&mut self) {
        // Collect player IDs first to avoid borrow issues
        let player_ids: Vec<Uuid> = self.world.players
            .iter()
            .filter(|(_, p)| p.spawned)
            .map(|(id, _)| *id)
            .collect();
        
        for player_id in player_ids {
            // 1. Send player update
            if let Some(player) = self.world.players.get(&player_id) {
                self.responses.push(GameResponse::PlayerUpdate {
                    player_id,
                    message: ServerMessage::PlayerUpdate {
                        data: player.to_update_data(),
                    },
                });
            }

            // 2. Send chunk data
            if let Some(interest) = self.world.interest_sets.get(&player_id).cloned() {
                let loaded = self.player_loaded_chunks.get(&player_id).cloned().unwrap_or_default();
                
                // Find new chunks to add
                let new_chunks: Vec<_> = interest.difference(&loaded).copied().collect();
                
                // Find old chunks to remove
                let removed_chunks: Vec<_> = loaded.difference(&interest).copied().collect();
                
                // Send chunkAdd for new chunks
                for coord in &new_chunks {
                    // Get or create chunk
                    let chunk = self.world.get_or_create_chunk(*coord);
                    
                    let chunk_data = ChunkAddData {
                        coord: [coord.0, coord.1],
                        biome: chunk.biome_id.clone(),
                        entities: chunk_to_entities(chunk),
                    };
                    
                    self.responses.push(GameResponse::ToPlayer {
                        player_id,
                        message: ServerMessage::ChunkAdd { data: chunk_data },
                    });
                }
                
                // Send chunkRemove for old chunks
                for coord in &removed_chunks {
                    self.responses.push(GameResponse::ToPlayer {
                        player_id,
                        message: ServerMessage::ChunkRemove {
                            data: ChunkRemoveData { coord: [coord.0, coord.1] },
                        },
                    });
                }
                
                // Update loaded chunks
                let mut new_loaded = interest.clone();
                // Remove chunks that were sent as removed
                for coord in &removed_chunks {
                    new_loaded.remove(coord);
                }
                self.player_loaded_chunks.insert(player_id, new_loaded);
            }
        }
    }

    pub fn get_player(&self, id: Uuid) -> Option<&PlayerState> {
        self.world.players.get(&id)
    }

    pub fn add_player(&mut self, id: Uuid, name: String) {
        self.world.add_player(id, name);
    }

    pub fn remove_player(&mut self, id: Uuid) {
        self.world.remove_player(id);
        self.player_loaded_chunks.remove(&id);
    }

    /// Process player input with properly split borrows to satisfy borrow checker
    fn process_player_input(
        world: &mut World,
        player_id: Uuid,
        data: &protocol::InputData,
        config: &std::sync::Arc<config::GameConfig>,
        _responses: &mut Vec<GameResponse>,
    ) {
        // Temporarily remove player from world to avoid borrow issues
        if let Some(mut player) = world.players.remove(&player_id) {
            InteractionSystem::handle_input(
                &mut player,
                data,
                world,
                &config.balance,
                1.0 / config.server.tick_rate as f64,
            );
            world.players.insert(player_id, player);
        }
    }
}

/// Convert a chunk to ChunkEntities for protocol
fn chunk_to_entities(chunk: &world::Chunk) -> ChunkEntities {
    ChunkEntities {
        resources: chunk.resources.iter().map(entity_to_snapshot).collect(),
        mobs: chunk.mobs.iter().map(entity_to_snapshot).collect(),
        structures: chunk.structures.iter().map(entity_to_snapshot).collect(),
        npcs: chunk.npcs.iter().map(entity_to_snapshot).collect(),
    }
}

/// Convert a world entity to a protocol EntitySnapshot
fn entity_to_snapshot(entity: &world::Entity) -> EntitySnapshot {
    EntitySnapshot {
        id: entity.id.clone(),
        kind: entity.kind.as_str().to_string(),
        subtype: entity.subtype.clone(),
        x: entity.x,
        y: entity.y,
        hp: if entity.hp < entity.max_hp { Some(entity.hp) } else { None },
        max_hp: Some(entity.max_hp),
        level: if entity.level > 1 { Some(entity.level) } else { None },
        name: entity.name.clone(),
        range: entity.range,
    }
}

pub struct GameHandle {
    inner: Arc<RwLock<GameEngine>>,
}

impl GameHandle {
    pub fn new(engine: GameEngine) -> Self {
        Self {
            inner: Arc::new(RwLock::new(engine)),
        }
    }
}

impl Clone for GameHandle {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl GameHandle {
    /// Enqueue a game event for processing on the next tick
    pub async fn enqueue(&self, event: GameEvent) -> Result<(), GameEvent> {
        let mut engine = self.inner.write().await;
        engine.enqueue_event(event)
    }

    pub async fn tick(&self, dt: f64) -> Vec<GameResponse> {
        let mut engine = self.inner.write().await;
        engine.tick(dt);
        engine.drain_responses()
    }

    pub async fn get_player(&self, id: Uuid) -> Option<PlayerState> {
        let engine = self.inner.read().await;
        engine.get_player(id).cloned()
    }
}

// Note: GameEngine is not Clone - use GameHandle for shared access
