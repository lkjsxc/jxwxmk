use actix::prelude::*;
use std::time::Duration;
use uuid::Uuid;
use std::collections::HashMap;
use crate::game::world_state::{World, Player, PlayerStats, Inventory, NpcType, ItemType, Structure};
use crate::config::AppConfig;
use crate::server::protocol::{ServerMessage, ClientMessage, NpcInteractionData, QuestUpdateData};
use crate::server::database::{DbPool, load_player, save_player, load_structures, save_structures};
use crate::game::quests::{QuestState, QuestStatus, QuestEvent, Objective, get_quest_definitions, check_progress};

pub struct GameEngine {
    tick_rate: u64,
    world: World,
    sessions: HashMap<Uuid, Recipient<ServerMessage>>,
    db_pool: DbPool,
    ticks: u64,
}

impl GameEngine {
    pub fn new(tick_rate: u64, db_pool: DbPool) -> Self {
        let config = AppConfig::get();
        let mut world = World {
            width: config.game.world_width,
            height: config.game.world_height,
            ..Default::default()
        };
        crate::game::spawning_and_ai::spawn_initial_entities(&mut world);
        Self { 
            tick_rate, 
            world,
            sessions: HashMap::new(),
            db_pool,
            ticks: 0,
        }
    }

    fn tick(&mut self, ctx: &mut Context<Self>) {
        self.ticks += 1;

        // 1. Barrier checks
        crate::game::systems_barriers::tick_barriers(&mut self.world);
        
        // 2. Survival tick
        for player in self.world.players.values_mut() {
            crate::game::systems_survival::tick_survival(player);
        }

        // 3. Mob AI
        crate::game::systems_ai::tick_mob_ai(&mut self.world);

        // 4. Mob damage
        crate::game::systems_ai::calculate_mob_damage_to_player(&mut self.world);

        // 5. Achievement Check (after damage/deaths)
        for player in self.world.players.values_mut() {
            let newly_unlocked = crate::game::achievements::check_achievements(player);
            for ach in newly_unlocked {
                if let Some(addr) = self.sessions.get(&player.id) {
                    let _ = addr.do_send(ServerMessage::Achievement { data: ach });
                }
            }
        }

        // 6. Tick effects
        let mut expired = Vec::new();
        for effect in self.world.effects.values_mut() {
            if effect.ttl > 0 {
                effect.ttl -= 1;
                effect.y -= 1.0; // Float up
            } else {
                expired.push(effect.id);
            }
        }
        for id in expired {
            self.world.effects.remove(&id);
        }

        // 7. Death cleanup
        
        // 6. Broadcast world snapshot
        self.broadcast_world();
        
        // 7. Periodic persistence (every 10 seconds approx)
        if self.ticks % (self.tick_rate * 10) == 0 {
            self.save_state();
        }

        ctx.run_later(Duration::from_millis(1000 / self.tick_rate), |act, ctx| {
            act.tick(ctx);
        });
    }

    fn save_state(&self) {
        // Save Players
        for player in self.world.players.values() {
            if player.spawned {
                let pool = self.db_pool.clone();
                let p = player.clone();
                actix::spawn(async move {
                    if let Err(e) = save_player(&pool, &p).await {
                        log::error!("Failed to save player {}: {}", p.id, e);
                    }
                });
            }
        }
        
        // Save World Structures
        let pool = self.db_pool.clone();
        let structures = self.world.structures.clone();
        actix::spawn(async move {
            if let Err(e) = save_structures(&pool, &structures).await {
                log::error!("Failed to save structures: {}", e);
            }
        });
    }

    fn broadcast_world(&mut self) {
        let spawned_players: HashMap<Uuid, Player> = self.world.players.iter()
            .filter(|(_, p)| p.spawned)
            .map(|(id, p)| (*id, p.clone()))
            .collect();

        let world_snapshot = World {
            width: self.world.width,
            height: self.world.height,
            players: spawned_players,
            resources: self.world.resources.clone(),
            mobs: self.world.mobs.clone(),
            structures: self.world.structures.clone(),
            npcs: self.world.npcs.clone(),
            barrier_cores: self.world.barrier_cores.clone(),
            effects: self.world.effects.clone(),
        };

        let msg = ServerMessage::World { data: world_snapshot };
        for recipient in self.sessions.values() {
            let _ = recipient.do_send(msg.clone());
        }
    }

    fn send_quest_update(&self, player_id: Uuid, quest: &QuestState) {
        if let Some(addr) = self.sessions.get(&player_id) {
            let defs = get_quest_definitions();
            if let Some(def) = defs.get(&quest.id) {
                let mut objectives = Vec::new();
                for obj in &quest.objectives {
                    objectives.push(serde_json::to_value(obj).unwrap());
                }

                let _ = addr.do_send(ServerMessage::QuestUpdate {
                    data: QuestUpdateData {
                        id: quest.id.clone(),
                        name: def.name.clone(),
                        description: def.description.clone(),
                        state: format!("{:?}", quest.status),
                        objectives,
                    }
                });
            }
        }
    }
}

impl Actor for GameEngine {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!("Game Engine started at {} Hz", self.tick_rate);
        
        // Load persistent structures
        let pool = self.db_pool.clone();
        let addr = ctx.address();
        
        actix::spawn(async move {
            match load_structures(&pool).await {
                Ok(structures) => {
                    addr.do_send(StructuresLoaded(structures));
                }
                Err(e) => {
                    log::error!("Failed to load structures: {}", e);
                }
            }
        });

        self.tick(ctx);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
struct StructuresLoaded(HashMap<Uuid, Structure>);

impl Handler<StructuresLoaded> for GameEngine {
    type Result = ();

    fn handle(&mut self, msg: StructuresLoaded, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("Loaded {} structures from DB", msg.0.len());
        self.world.structures.extend(msg.0);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    pub id: Uuid,
    pub token: Option<Uuid>,
    pub addr: Recipient<ServerMessage>,
}

impl Handler<Join> for GameEngine {
    type Result = ();

    fn handle(&mut self, msg: Join, ctx: &mut Self::Context) -> Self::Result {
        let pool = self.db_pool.clone();
        let addr = msg.addr.clone();
        let session_id = msg.id;
        let provided_token = msg.token;
        let config = AppConfig::get();
        let default_x = config.game.world_width / 2.0;
        let default_y = config.game.world_height / 2.0;
        let max_hp = config.balance.player.max_health;
        let max_hunger = config.balance.player.max_hunger;

        let recipient = ctx.address().recipient();

        actix::spawn(async move {
            let player_res = if let Some(token) = provided_token {
                load_player(&pool, token).await
            } else {
                Ok(None)
            };

            match player_res {
                Ok(Some(player)) => {
                    recipient.do_send(FinishJoin {
                        session_id,
                        player,
                        addr,
                        is_new: false,
                    });
                }
                Ok(None) => {
                    let new_id = Uuid::new_v4();
                    let new_token = Uuid::new_v4();
                    let player = Player {
                        id: new_id,
                        token: new_token,
                        username: "Guest".to_string(),
                        x: default_x,
                        y: default_y,
                        health: max_hp,
                        hunger: max_hunger,
                        cold: 0.0,
                        inventory: Inventory::new(30),
                        active_slot: 0,
                        stats: PlayerStats::default(),
                        achievements: std::collections::HashSet::new(),
                        stat_bonuses: HashMap::new(),
                        quests: HashMap::new(),
                        spawned: false,
                        last_attack_at: 0.0,
                        last_interact_at: 0.0,
                    };
                    if let Err(e) = save_player(&pool, &player).await {
                        log::error!("Failed to create new player: {}", e);
                    }
                    recipient.do_send(FinishJoin {
                        session_id,
                        player,
                        addr,
                        is_new: true,
                    });
                }
                Err(e) => {
                    log::error!("DB Error on join: {}", e);
                }
            }
        });
    }
}

#[derive(Message)]
#[rtype(result = "()")]
struct FinishJoin {
    session_id: Uuid,
    player: Player,
    addr: Recipient<ServerMessage>,
    is_new: bool,
}

impl Handler<FinishJoin> for GameEngine {
    type Result = ();

    fn handle(&mut self, msg: FinishJoin, _ctx: &mut Self::Context) -> Self::Result {
        self.sessions.insert(msg.player.id, msg.addr.clone());
        self.world.players.insert(msg.player.id, msg.player.clone());
        
        let _ = msg.addr.do_send(ServerMessage::Welcome {
            id: msg.player.id,
            token: msg.player.token,
            spawned: msg.player.spawned,
        });

        // Send active quest states
        for quest in msg.player.quests.values() {
             self.send_quest_update(msg.player.id, quest);
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Leave {
    pub id: Uuid,
}

impl Handler<Leave> for GameEngine {
    type Result = ();

    fn handle(&mut self, msg: Leave, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(player) = self.world.players.get(&msg.id) {
             let pool = self.db_pool.clone();
             let p = player.clone();
             actix::spawn(async move {
                 let _ = save_player(&pool, &p).await;
             });
        }
        self.sessions.remove(&msg.id);
        self.world.players.remove(&msg.id);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientInput {
    pub id: Uuid,
    pub msg: ClientMessage,
}

impl Handler<ClientInput> for GameEngine {
    type Result = ();

    fn handle(&mut self, msg: ClientInput, _ctx: &mut Self::Context) -> Self::Result {
        let mut attack_req = None;
        let mut craft_req = None;
        let mut interact_req = false;
        let mut npc_action_req = None;
        let mut player_clone_for_interact = None;

        // 1. First Pass: Read Input & Mutate Player (Spawn, Move)
        if let Some(player) = self.world.players.get_mut(&msg.id) {
            if let Some(spawn) = msg.msg.spawn {
                if spawn && !player.spawned {
                    player.spawned = true;
                    if player.health <= 0.0 {
                         player.health = AppConfig::get().balance.player.max_health;
                         player.hunger = AppConfig::get().balance.player.max_hunger;
                    }
                }
            }

            if player.spawned {
                let speed = AppConfig::get().balance.player.base_speed;
                let dx = msg.msg.dx.unwrap_or(0.0);
                let dy = msg.msg.dy.unwrap_or(0.0);
                
                let len = (dx * dx + dy * dy).sqrt();
                if len > 0.0 {
                    player.x += (dx / len) * speed;
                    player.y += (dy / len) * speed;
                    player.stats.steps += 1;
                    
                    let newly_unlocked = crate::game::achievements::check_achievements(player);
                    for ach in newly_unlocked {
                        if let Some(addr) = self.sessions.get(&player.id) {
                            let _ = addr.do_send(ServerMessage::Achievement { data: ach });
                        }
                    }
                }

                player.x = player.x.clamp(0.0, self.world.width);
                player.y = player.y.clamp(0.0, self.world.height);

                if let Some(true) = msg.msg.attack {
                    attack_req = Some(player.clone());
                }
                
                if let Some(item_type) = msg.msg.craft {
                    craft_req = Some(item_type);
                }

                if let Some(true) = msg.msg.interact {
                    interact_req = true;
                    player_clone_for_interact = Some(player.clone());
                }

                if let Some(action) = msg.msg.npc_action {
                    npc_action_req = Some(action);
                }

                if let Some(slot) = msg.msg.slot {
                    if slot < 7 { // Hotbar is 0-6
                        player.active_slot = slot;
                    }
                }

                if let Some((from, to)) = msg.msg.swap_slots {
                    if from < player.inventory.slots.len() && to < player.inventory.slots.len() {
                        player.inventory.slots.swap(from, to);
                    }
                }
            }
        }

        // 2. Second Pass: World Interactions (Attack, Interact)
        
        // Attack / Gather
        if let Some(mut p_clone) = attack_req {
            let events = crate::game::systems_interaction::handle_attack(&mut p_clone, &mut self.world);
            
            // Sync Player
            let mut updates_to_send = Vec::new();
            let mut ach_to_send = Vec::new();
            
            if let Some(real_player) = self.world.players.get_mut(&msg.id) {
                real_player.stats = p_clone.stats;
                real_player.inventory = p_clone.inventory;
                
                // Process Quest Events
                for event in events {
                    let mut updates: Vec<QuestState> = Vec::new();
                    for quest in real_player.quests.values_mut() {
                        if check_progress(quest, &event) {
                            updates.push(quest.clone());
                        }
                    }
                    updates_to_send.extend(updates);
                }

                ach_to_send = crate::game::achievements::check_achievements(real_player);
            }
            
            for quest in updates_to_send {
                self.send_quest_update(msg.id, &quest);
            }
            for ach in ach_to_send {
                if let Some(addr) = self.sessions.get(&msg.id) {
                    let _ = addr.do_send(ServerMessage::Achievement { data: ach });
                }
            }
        }

        // Interaction (NPCs)
        if interact_req {
            if let Some(p) = player_clone_for_interact {
                if let Some(npc_id) = crate::game::systems_interaction::handle_interact(&p, &self.world) {
                    if let Some(npc) = self.world.npcs.get(&npc_id) {
                        // Send interaction dialog
                        // Determine options based on quest state
                        // Simplified: Just one set of options for now
                        let mut options = Vec::new();
                        let mut text = "Greetings.".to_string();

                        if let Some(player) = self.world.players.get(&msg.id) {
                             match npc.n_type {
                                 NpcType::Elder => {
                                     if !player.quests.contains_key("wood_gatherer") {
                                         text = "We need wood for the fires. Can you help?".to_string();
                                         options.push("I need a quest.".to_string()); // Index 0
                                     } else {
                                         let q = player.quests.get("wood_gatherer").unwrap();
                                         if q.status == QuestStatus::ReadyToTurnIn {
                                             text = "Ah, you have the wood!".to_string();
                                             options.push("Complete Quest".to_string()); // Index 0
                                         } else if q.status == QuestStatus::Completed {
                                             text = "Thank you for your help.".to_string();
                                             if !player.quests.contains_key("wolf_hunter") {
                                                 options.push("Any more work?".to_string()); // Index 0
                                             }
                                         } else {
                                             text = "Please hurry with the wood.".to_string();
                                         }
                                     }
                                 },
                                 _ => {
                                     options.push("Trade".to_string());
                                 }
                             }
                        }
                        options.push("Goodbye".to_string());

                        if let Some(addr) = self.sessions.get(&msg.id) {
                            let _ = addr.do_send(ServerMessage::NpcInteraction {
                                data: NpcInteractionData {
                                    npc_id: npc.id,
                                    npc_type: format!("{:?}", npc.n_type),
                                    name: npc.name.clone(),
                                    text,
                                    options,
                                    trade_items: vec![],
                                }
                            });
                        }
                    }
                }
            }
        }

        // NPC Actions (Dialogue responses)
        if let Some((npc_id, option_idx)) = npc_action_req {
            let mut quest_update = None;
            
            if let Some(npc) = self.world.npcs.get(&npc_id) {
                 if let Some(player) = self.world.players.get_mut(&msg.id) {
                     match npc.n_type {
                         NpcType::Elder => {
                             // This is brittle mapping, ideal would be to store "dialogue_state" or similar.
                             // But for this simplified version:
                             let wood_quest = player.quests.get("wood_gatherer");
                             let wolf_quest = player.quests.get("wolf_hunter");

                             if wood_quest.is_none() {
                                 if option_idx == 0 { // "I need a quest"
                                     let defs = get_quest_definitions();
                                     if let Some(def) = defs.get("wood_gatherer") {
                                         let new_quest = QuestState {
                                             id: def.id.clone(),
                                             status: QuestStatus::InProgress,
                                             objectives: def.objectives.clone(),
                                         };
                                         player.quests.insert(def.id.clone(), new_quest.clone());
                                         quest_update = Some(new_quest);
                                     }
                                 }
                             } else if let Some(q) = wood_quest {
                                 if q.status == QuestStatus::ReadyToTurnIn && option_idx == 0 {
                                      // Complete it
                                      if let Some(q_mut) = player.quests.get_mut("wood_gatherer") {
                                          q_mut.status = QuestStatus::Completed;
                                          // Reward logic: Remove wood? Add food?
                                          // Removing items is complex here as we need `Inventory`. 
                                          // Just marked complete for now.
                                          quest_update = Some(q_mut.clone());
                                      }
                                 } else if q.status == QuestStatus::Completed && wolf_quest.is_none() && option_idx == 0 {
                                     // Start wolf quest
                                     let defs = get_quest_definitions();
                                     if let Some(def) = defs.get("wolf_hunter") {
                                         let new_quest = QuestState {
                                             id: def.id.clone(),
                                             status: QuestStatus::InProgress,
                                             objectives: def.objectives.clone(),
                                         };
                                         player.quests.insert(def.id.clone(), new_quest.clone());
                                         quest_update = Some(new_quest);
                                     }
                                 }
                             }
                         },
                         _ => {}
                     }
                 }
            }
            
            if let Some(q) = quest_update {
                self.send_quest_update(msg.id, &q);
            }
        }

        // 3. Third Pass: Crafting
        if let Some(item_type) = craft_req {
             if let Some(player) = self.world.players.get_mut(&msg.id) {
                 if crate::game::systems_crafting::handle_craft(player, item_type) {
                     // Inventory updated
                     let newly_unlocked = crate::game::achievements::check_achievements(player);
                     for ach in newly_unlocked {
                         if let Some(addr) = self.sessions.get(&player.id) {
                             let _ = addr.do_send(ServerMessage::Achievement { data: ach });
                         }
                     }
                 }
             }
        }
    }
}
