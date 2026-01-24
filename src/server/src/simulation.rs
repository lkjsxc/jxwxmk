use std::time::{Duration, Instant};
use tokio::sync::{mpsc, broadcast};
use tracing::{info, debug, error};
use crate::config::GameSettings;
use crate::world::{GameWorld, WorldEntity, EntityType, Inventory};
use crate::systems::{MovementSystem, ResourceSystem, SurvivalSystem, CombatSystem, CraftingSystem};

pub struct GameSimulation {
    config: GameSettings,
    tick_rate: u32,
    tick_duration: Duration,
    last_tick: Instant,
    current_tick: u64,
    running: bool,
    pub event_receiver: mpsc::Receiver<GameEvent>,
    pub event_sender: mpsc::Sender<GameEvent>,
    pub world: GameWorld,
    pub state_broadcast: broadcast::Sender<crate::protocol::ServerMessage>,
}

#[derive(Debug, Clone)]
pub enum GameEvent {
    PlayerInput { player_id: String, input: PlayerInput },
    PlayerConnected { player_id: String },
    PlayerDisconnected { player_id: String },
    SystemEvent { event_type: String },
}

#[derive(Debug, Clone, Default)]
pub struct PlayerInput {
    pub movement: MovementInput,
    pub actions: Vec<PlayerAction>,
    pub sequence: u32,
}

#[derive(Debug, Clone, Default)]
pub struct MovementInput {
    pub direction: (f32, f32),
    pub speed: f32,
    pub sprinting: bool,
}

#[derive(Debug, Clone)]
pub enum PlayerAction {
    Attack,
    UseItem { slot: usize },
    Craft { recipe_id: String },
    Interact,
}

impl GameSimulation {
    pub fn new(config: GameSettings, state_broadcast: broadcast::Sender<crate::protocol::ServerMessage>) -> Self {
        let tick_rate = config.tick_rate;
        let tick_duration = Duration::from_secs_f32(1.0 / tick_rate as f32);
        
        let (event_sender, event_receiver) = mpsc::channel(100);
        
        let mut world = GameWorld::new(config.world_size, config.world_size);
        world.generate_world();
        
        Self {
            config,
            tick_rate,
            tick_duration,
            last_tick: Instant::now(),
            current_tick: 0,
            running: false,
            event_receiver,
            event_sender,
            world,
            state_broadcast,
        }
    }
    
    pub fn start(&mut self) {
        self.running = true;
        self.last_tick = Instant::now();
        info!("Game simulation started with tick rate: {} Hz", self.tick_rate);
    }
    
    pub fn stop(&mut self) {
        self.running = false;
        info!("Game simulation stopped");
    }
    
    pub async fn tick(&mut self) {
        if !self.running {
            return;
        }
        
        let now = Instant::now();
        if now.duration_since(self.last_tick) < self.tick_duration {
            return;
        }
        
        self.last_tick = now;
        self.current_tick += 1;
        
        self.process_events().await;
        self.update_game_state(self.tick_duration.as_secs_f32()).await;
        self.send_state_updates().await;
    }
    
    async fn process_events(&mut self) {
        while let Ok(event) = self.event_receiver.try_recv() {
            match event {
                GameEvent::PlayerInput { player_id, input } => {
                    self.handle_player_input(&player_id, input);
                }
                GameEvent::PlayerConnected { player_id } => {
                    info!("Player connected: {}", player_id);
                    let player = WorldEntity {
                        id: player_id.clone(),
                        entity_type: EntityType::Player,
                        position: (100.0, 100.0),
                        velocity: (0.0, 0.0),
                        rotation: 0.0,
                        health: 100.0,
                        max_health: 100.0,
                        inventory: Inventory::default(),
                        last_sequence: 0,
                    };
                    self.world.add_entity(player);
                }
                GameEvent::PlayerDisconnected { player_id } => {
                    info!("Player disconnected: {}", player_id);
                    self.world.entities.remove(&player_id);
                }
                GameEvent::SystemEvent { event_type } => {
                    debug!("System event: {}", event_type);
                }
            }
        }
    }
    
    fn handle_player_input(&mut self, player_id: &str, input: PlayerInput) {
        if let Some(player) = self.world.entities.get_mut(player_id) {
            player.last_sequence = input.sequence;
            
            player.velocity = MovementSystem::calculate_velocity(
                input.movement.direction,
                input.movement.speed,
                input.movement.sprinting,
            );
            
            if input.movement.direction.0 != 0.0 || input.movement.direction.1 != 0.0 {
                player.rotation = input.movement.direction.1.atan2(input.movement.direction.0);
            }
        }

        for action in input.actions {
            match action {
                PlayerAction::Interact => self.handle_interact(player_id),
                PlayerAction::Attack => self.handle_attack(player_id),
                PlayerAction::Craft { recipe_id } => self.handle_craft(player_id, recipe_id),
                _ => {}
            }
        }
    }

    fn handle_interact(&mut self, player_id: &str) {
        let player_pos = if let Some(player) = self.world.entities.get(player_id) {
            player.position
        } else {
            return;
        };

        let mut nearest_resource_id: Option<String> = None;
        let mut min_dist = f32::MAX;
        let interaction_range = 50.0;

        for resource in self.world.resources.values() {
             let dist = ((resource.position.0 - player_pos.0).powi(2) + (resource.position.1 - player_pos.1).powi(2)).sqrt();
             if dist < interaction_range && dist < min_dist && resource.quantity > 0.0 {
                 min_dist = dist;
                 nearest_resource_id = Some(resource.id.clone());
             }
        }

        if let Some(res_id) = nearest_resource_id {
            let resource = self.world.resources.get_mut(&res_id);
            let player = self.world.entities.get_mut(player_id);
            
            if let (Some(res), Some(pl)) = (resource, player) {
                 if let Some((item, amount)) = ResourceSystem::gather(res, &mut pl.inventory, 10.0) {
                     info!("Player {} gathered {} x{}", player_id, item, amount);
                 }
            }
        }
    }

    fn handle_attack(&mut self, player_id: &str) {
        let (attacker_pos, attacker_rot) = if let Some(p) = self.world.entities.get(player_id) {
            (p.position, p.rotation)
        } else {
            return;
        };

        let attack_range = 50.0;
        let attack_angle = std::f32::consts::PI / 2.0;
        let damage = 10.0;

        let mut hit_ids = Vec::new();

        for entity in self.world.entities.values() {
            if entity.id == player_id { continue; }

            let dx = entity.position.0 - attacker_pos.0;
            let dy = entity.position.1 - attacker_pos.1;
            let dist_sq = dx*dx + dy*dy;

            if dist_sq < attack_range * attack_range {
                let angle_to_target = dy.atan2(dx);
                let angle_diff = (angle_to_target - attacker_rot).abs();
                let normalized_diff = if angle_diff > std::f32::consts::PI {
                    2.0 * std::f32::consts::PI - angle_diff
                } else {
                    angle_diff
                };

                if normalized_diff < attack_angle / 2.0 {
                    hit_ids.push(entity.id.clone());
                }
            }
        }

        for target_id in hit_ids {
            if let Some(target) = self.world.entities.get_mut(&target_id) {
                if CombatSystem::apply_damage(&mut target.health, damage) {
                    info!("Entity {} killed by {}", target_id, player_id);
                } else {
                    info!("Entity {} took {} damage from {}", target_id, damage, player_id);
                }
            }
        }
    }

    fn handle_craft(&mut self, player_id: &str, recipe_id: String) {
        if let Some(player) = self.world.entities.get_mut(player_id) {
            let recipes = CraftingSystem::get_default_recipes();
            if let Some(recipe) = recipes.iter().find(|r| r.id == recipe_id) {
                if CraftingSystem::perform_craft(recipe, &mut player.inventory) {
                    info!("Player {} crafted {}", player_id, recipe.name);
                } else {
                    debug!("Player {} failed to craft {}: insufficient resources", player_id, recipe.name);
                }
            }
        }
    }
    
    async fn update_game_state(&mut self, delta_time: f32) {
        for entity in self.world.entities.values_mut() {
            MovementSystem::update_position(
                &mut entity.position,
                entity.velocity,
                delta_time,
                1.0,
            );
        }
        
        self.world.update(delta_time);
    }
    
    async fn send_state_updates(&self) {
        let entities: Vec<crate::protocol::EntityState> = self.world.entities.values().map(|e| {
            crate::protocol::EntityState {
                id: e.id.clone(),
                position: e.position,
                velocity: e.velocity,
                rotation: e.rotation,
                health: e.health,
                max_health: e.max_health,
                entity_type: format!("{:?}", e.entity_type),
                last_sequence: e.last_sequence,
            }
        }).collect();

        let msg = crate::protocol::ServerMessage::StateUpdate {
            tick: self.current_tick,
            entities,
        };
        
        let _ = self.state_broadcast.send(msg);
    }
    
    pub fn current_tick(&self) -> u64 {
        self.current_tick
    }
    
    pub fn is_running(&self) -> bool {
        self.running
    }
}