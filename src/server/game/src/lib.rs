use config::Config;
use protocol::ClientMessage;
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::time::interval;
use world::{PlayerId, PlayerState, World};
use world::biome::BiomeGenerator;
use world::spawning::ResourceSpawner;

pub struct GameEngine {
    world: Arc<RwLock<World>>,
    config: Arc<Config>,
    event_queue: VecDeque<GameEvent>,
    biome_generator: BiomeGenerator,
    resource_spawner: ResourceSpawner,
    tick_count: u64,
}

#[derive(Debug)]
pub enum GameEvent {
    Join(PlayerId, String),
    Leave(PlayerId),
    Input(PlayerId, ClientMessage),
}

impl GameEngine {
    pub fn new(config: Config) -> Self {
        let seed = config.world.seed.unwrap_or_else(|| rand::random::<u64>());
        let world = World::new(seed);

        GameEngine {
            world: Arc::new(RwLock::new(world)),
            config: Arc::new(config),
            event_queue: VecDeque::new(),
            biome_generator: BiomeGenerator::new(seed),
            resource_spawner: ResourceSpawner::new(seed),
            tick_count: 0,
        }
    }

    pub fn enqueue_event(&mut self, event: GameEvent) {
        if self.event_queue.len() < 10000 {
            self.event_queue.push_back(event);
        } else {
            log::warn!("Event queue full, dropping event");
        }
    }

    pub async fn run(mut self) {
        let tick_duration = Duration::from_secs_f32(1.0 / self.config.server.tick_rate as f32);
        let mut tick_interval = interval(tick_duration);

        log::info!(
            "Starting game loop at {} Hz",
            self.config.server.tick_rate
        );

        loop {
            tick_interval.tick().await;
            let tick_start = Instant::now();
            self.tick().await;
            let tick_duration_ms = tick_start.elapsed().as_millis() as u64;
            
            // Log tick performance
            if tick_duration_ms > 50 {
                log::warn!("Slow tick: {}ms", tick_duration_ms);
            }
        }
    }

    async fn tick(&mut self) {
        let dt = 1.0 / self.config.server.tick_rate as f32;
        self.tick_count += 1;

        // Process events
        while let Some(event) = self.event_queue.pop_front() {
            self.handle_event(event).await;
        }

        let mut world = self.world.write().await;

        // Update active chunks and spawn resources
        let active_chunks: Vec<_> = world.active_chunks.iter().copied().collect();
        for coord in active_chunks {
            if let Some(chunk) = world.chunks.get_mut(&coord) {
                self.resource_spawner.update(chunk, dt);
            }
        }

        // Collect player chunk positions first
        let player_chunks: Vec<_> = world
            .players
            .values()
            .filter(|p| p.spawned)
            .map(|p| (p.id, world::pos_to_chunk((p.x, p.y))))
            .collect();

        // Update players
        for player in world.players.values_mut() {
            if player.spawned {
                // Get biome at player position
                let player_chunk = world::pos_to_chunk((player.x, player.y));
                let biome = self
                    .biome_generator
                    .generate_biome(player_chunk);

                systems::SurvivalSystem::tick(
                    player,
                    dt,
                    &self.config.survival,
                    &self.config.balance,
                    Some(&biome),
                );

                if systems::DeathSystem::check_death(player) {
                    log::info!("Player {} died", player.id);
                }

                let new_achievements = systems::AchievementSystem::check_achievements(player);
                for achievement in new_achievements {
                    log::info!("Player {} earned achievement: {}", player.id, achievement);
                }
            }
        }

        // Periodic cleanup (every 60 ticks = ~2 seconds at 30Hz)
        if self.tick_count % 60 == 0 {
            world.clear_dirty_chunks();
        }
    }

    async fn handle_event(&mut self, event: GameEvent) {
        let mut world = self.world.write().await;

        match event {
            GameEvent::Join(player_id, name) => {
                let player = PlayerState::new(player_id, name);
                world.players.insert(player_id, player);
                log::info!("Player {} joined", player_id);
            }
            GameEvent::Leave(player_id) => {
                world.players.remove(&player_id);
                world.interest_sets.remove(&player_id);
                log::info!("Player {} left", player_id);
            }
            GameEvent::Input(player_id, message) => {
                if let Some(player) = world.players.get_mut(&player_id) {
                    self.handle_player_input(player, message).await;
                }
            }
        }
    }

    async fn handle_player_input(&self,
        player: &mut PlayerState,
        message: ClientMessage,
    ) {
        match message {
            ClientMessage::Input(data) => {
                systems::MovementSystem::apply_movement(
                    player,
                    data.dx,
                    data.dy,
                    1.0 / self.config.server.tick_rate as f32,
                    &self.config.balance,
                );
            }
            ClientMessage::Spawn(_) => {
                if !player.spawned {
                    player.spawned = true;
                    player.x = 0.0;
                    player.y = 0.0;
                    player.vitals.hp = player.vitals.max_hp;
                    log::info!("Player {} spawned", player.id);
                }
            }
            ClientMessage::Craft(data) => {
                if let Err(e) =
                    systems::CraftingSystem::craft(player, &data.recipe, &self.config.crafting.recipes)
                {
                    log::warn!("Crafting failed for player {}: {}", player.id, e);
                }
            }
            _ => {}
        }
    }

    pub fn get_world_handle(&self) -> WorldHandle {
        WorldHandle {
            world: Arc::clone(&self.world),
        }
    }

    pub fn get_stats(&self) -> GameStats {
        GameStats {
            tick_count: self.tick_count,
            event_queue_len: self.event_queue.len(),
        }
    }
}

#[derive(Clone)]
pub struct WorldHandle {
    world: Arc<RwLock<World>>,
}

impl WorldHandle {
    pub async fn get_player(&self,
        id: PlayerId,
    ) -> Option<PlayerState> {
        let world = self.world.read().await;
        world.players.get(&id).cloned()
    }

    pub async fn get_player_count(&self) -> usize {
        let world = self.world.read().await;
        world.players.len()
    }

    pub async fn get_active_chunk_count(&self) -> usize {
        let world = self.world.read().await;
        world.active_chunks.len()
    }
}

pub struct GameStats {
    pub tick_count: u64,
    pub event_queue_len: usize,
}
