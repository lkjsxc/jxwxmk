use crate::events::GameEvent;
use actix::prelude::*;
use config::Config;
use std::collections::{VecDeque, HashMap};
use std::time::{Duration, Instant};
use world::{World, ChunkCoord};
use uuid::Uuid;
use protocol::{ServerMessage, NotificationData};
use persistence::PersistenceManager;

use crate::metrics;
use crate::interest;
use crate::deltas;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage(pub GameEvent);

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct OutboundMessage(pub String);

impl From<String> for OutboundMessage {
    fn from(s: String) -> Self {
        OutboundMessage(s)
    }
}

#[derive(Message)]
#[rtype(result = "String")]
pub struct GetMetrics;

pub struct GameEngine {
    pub world: World,
    pub event_queue: VecDeque<GameEvent>,
    pub sessions: HashMap<Uuid, Recipient<OutboundMessage>>,
    pub persistence: Option<PersistenceManager>,
    pub metrics: metrics::EngineMetrics,
    pub last_checkpoint: Instant,
    pub tick_rate: u32,
    pub tick_count: u64,
}

impl GameEngine {
    pub fn new(config: Config, persistence: Option<PersistenceManager>) -> Self {
        let tick_rate = config.server.tick_rate;
        Self {
            world: World::new(config),
            event_queue: VecDeque::new(),
            sessions: HashMap::new(),
            persistence,
            metrics: metrics::EngineMetrics::new(),
            last_checkpoint: Instant::now(),
            tick_rate,
            tick_count: 0,
        }
    }

    pub fn enqueue(&mut self, event: GameEvent) {
        if self.event_queue.len() >= 1000 {
            self.metrics.input_dropped.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let _ = self.event_queue.pop_front();
        }
        self.event_queue.push_back(event);
        self.metrics.input_queue_len.store(self.event_queue.len(), std::sync::atomic::Ordering::Relaxed);
    }

    pub fn tick(&mut self, dt: Duration) {
        let start = Instant::now();
        self.tick_count += 1;

        let events_to_process = self.event_queue.len();
        for _ in 0..events_to_process {
            if let Some(event) = self.event_queue.pop_front() {
                self.handle_event(event);
            }
        }

        systems::interaction::tick(&mut self.world, dt);
        systems::survival::tick(&mut self.world, dt);
        systems::barrier::tick(&mut self.world, dt);
        systems::spawning::tick(&mut self.world, dt);
        systems::ai::tick(&mut self.world, dt);
        systems::death::tick(&mut self.world, dt);
        systems::achievements::tick(&mut self.world, dt);

        interest::update_interest(&mut self.world, &self.sessions);
        deltas::broadcast_deltas(&self.world, &self.sessions, self.tick_count);

        self.flush_notifications();

        if self.last_checkpoint.elapsed() > Duration::from_secs(30) {
            self.checkpoint();
            self.last_checkpoint = Instant::now();
        }

        self.metrics.tick_duration_ms.store(start.elapsed().as_millis() as u64, std::sync::atomic::Ordering::Relaxed);
        self.metrics.active_players.store(self.world.players.len(), std::sync::atomic::Ordering::Relaxed);
        self.metrics.active_chunks.store(self.world.chunks.len(), std::sync::atomic::Ordering::Relaxed);
    }

    fn flush_notifications(&mut self) {
        for (pid, text) in self.world.pending_notifications.drain(..) {
            if let Some(recipient) = self.sessions.get(&pid) {
                let msg = ServerMessage::Notification { data: NotificationData { text } };
                if let Ok(json) = serde_json::to_string(&msg) {
                    let _ = recipient.do_send(OutboundMessage(json));
                }
            }
        }
    }

    fn handle_event(&mut self, event: GameEvent) {
        match event {
            GameEvent::PlayerJoin { player_id, name, token, recipient } => {
                self.process_join(player_id, token, name, recipient);
            }
            GameEvent::PlayerRejoin { state, recipient } => {
                self.process_rejoin(state, recipient);
            }
            GameEvent::PlayerLeave { player_id } => {
                if let (Some(p), Some(pm)) = (self.world.players.get(&player_id), &self.persistence) {
                    let pool = pm.get_pool().clone();
                    let p_clone = p.clone();
                    actix::spawn(async move {
                        let _ = persistence::player::save_player(&pool, &p_clone).await;
                    });
                }
                self.world.players.remove(&player_id);
                self.sessions.remove(&player_id);
            }
            GameEvent::Spawn { player_id, settlement_id: _ } => {
                systems::death::respawn(&mut self.world, player_id);
            }
            GameEvent::Input { player_id, dx, dy, attack, interact, aim } => {
                if let Some(player) = self.world.players.get_mut(&player_id) {
                    player.input_dx = dx;
                    player.input_dy = dy;
                    player.input_attack = attack;
                    player.input_interact = interact;
                    player.input_aim = aim;
                }
            }
            GameEvent::Craft { player_id, recipe_id } => {
                if let Err(e) = systems::crafting::craft(&mut self.world, player_id, &recipe_id) {
                    if let Some(recipient) = self.sessions.get(&player_id) {
                        let msg = ServerMessage::Error {
                            data: protocol::ErrorData {
                                code: "insufficient_items".to_string(),
                                message: e,
                                details: None,
                            }
                        };
                        if let Ok(json) = serde_json::to_string(&msg) {
                            let _ = recipient.do_send(OutboundMessage(json));
                        }
                    }
                }
            }
            GameEvent::AcceptQuest { player_id, quest_id } => {
                let _ = systems::quests::accept_quest(&mut self.world, player_id, &quest_id);
            }
            GameEvent::Slot { player_id, slot } => {
                if let Some(p) = self.world.players.get_mut(&player_id) {
                    if slot < 7 { p.active_slot = slot as usize; }
                }
            }
            GameEvent::SwapSlots { player_id, from, to } => {
                if let Some(p) = self.world.players.get_mut(&player_id) {
                    if from < 28 && to < 28 {
                        p.inventory.swap(from as usize, to as usize);
                    }
                }
            }
        }
    }

    fn process_join(&mut self, player_id: Uuid, token: Uuid, name: String, recipient: Recipient<OutboundMessage>) {
        if let Some(old_recipient) = self.sessions.get(&player_id) {
            let revoke = ServerMessage::SessionRevoked { reason: "login_elsewhere".into() };
            if let Ok(json) = serde_json::to_string(&revoke) {
                let _ = old_recipient.do_send(OutboundMessage(json));
            }
        }

        let state = world::PlayerState {
            id: player_id,
            token,
            name,
            level: 1,
            xp: 0,
            pos: world::Vec2 { x: 64.0, y: 64.0 },
            chunk: ChunkCoord { x: 0, y: 0 },
            hp: 100.0,
            max_hp: 100.0,
            hunger: 100.0,
            thirst: 100.0,
            temp: 50.0,
            inventory: vec![None; 28],
            active_slot: 0,
            stats: HashMap::new(),
            unlocked_achievements: std::collections::HashSet::new(),
            stat_bonuses: HashMap::new(),
            active_quests: Vec::new(),
            spawned: false,
            active_view: std::collections::HashSet::new(),
            input_dx: 0.0,
            input_dy: 0.0,
            input_attack: false,
            input_interact: false,
            input_aim: None,
        };
        self.world.players.insert(player_id, state);
        self.sessions.insert(player_id, recipient.clone());

        let welcome = ServerMessage::Welcome {
            id: player_id,
            token,
            version: 3,
            spawned: false,
        };
        if let Ok(json) = serde_json::to_string(&welcome) {
            let _ = recipient.do_send(OutboundMessage(json));
        }
    }

    fn process_rejoin(&mut self, state: world::PlayerState, recipient: Recipient<OutboundMessage>) {
        let player_id = state.id;
        let token = state.token;
        let spawned = state.spawned;

        if let Some(old_recipient) = self.sessions.get(&player_id) {
            let revoke = ServerMessage::SessionRevoked { reason: "login_elsewhere".into() };
            if let Ok(json) = serde_json::to_string(&revoke) {
                let _ = old_recipient.do_send(OutboundMessage(json));
            }
        }

        self.world.players.insert(player_id, state);
        self.sessions.insert(player_id, recipient.clone());

        let welcome = ServerMessage::Welcome {
            id: player_id,
            token,
            version: 3,
            spawned,
        };
        if let Ok(json) = serde_json::to_string(&welcome) {
            let _ = recipient.do_send(OutboundMessage(json));
        }
    }

    fn checkpoint(&mut self) {
        if let Some(pm) = &self.persistence {
            let pool = pm.get_pool().clone();
            for p in self.world.players.values() {
                let p_clone = p.clone();
                let pool_clone = pool.clone();
                actix::spawn(async move {
                    let _ = persistence::player::save_player(&pool_clone, &p_clone).await;
                });
            }
            for chunk in self.world.chunks.values_mut() {
                if chunk.dirty {
                    let c_clone = chunk.clone();
                    let pool_clone = pool.clone();
                    actix::spawn(async move {
                        let _ = persistence::world_state::save_chunk(&pool_clone, &c_clone).await;
                    });
                    chunk.dirty = false;
                }
            }
            for s in self.world.settlements.values() {
                let s_clone = s.clone();
                let pool_clone = pool.clone();
                actix::spawn(async move {
                    let _ = persistence::world_state::save_settlement(&pool_clone, &s_clone).await;
                });
            }
        }
    }
}

impl Actor for GameEngine {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let tick_duration = Duration::from_secs_f64(1.0 / self.tick_rate as f64);
        ctx.run_interval(tick_duration, move |act, _ctx| {
            act.tick(tick_duration);
        });
    }
}

impl Handler<ClientMessage> for GameEngine {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _ctx: &mut Self::Context) {
        self.enqueue(msg.0);
    }
}

impl Handler<GetMetrics> for GameEngine {
    type Result = String;

    fn handle(&mut self, _msg: GetMetrics, _ctx: &mut Self::Context) -> String {
        self.metrics.to_prometheus()
    }
}