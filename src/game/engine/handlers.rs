use actix::prelude::*;
use uuid::Uuid;
use rand::Rng;
use crate::game::engine::GameEngine;
use crate::game::engine::messages::*;
use crate::game::entities::player::Player;
use crate::game::systems::crafting::CraftingSystem;
use crate::game::systems::interaction::{InteractionSystem, InteractionEvent};

use crate::game::systems::quests::{QuestSystem, QuestState};

impl Handler<Join> for GameEngine {
    type Result = Option<(String, Uuid, bool)>;
    fn handle(&mut self, msg: Join, _: &mut Context<Self>) -> Self::Result {
        if let Some(token) = msg.token { 
            if let Some(player) = self.world.players.values_mut().find(|p| p.token == token) { 
                let player_id = player.id; 
                let spawned = player.spawned;
                self.sessions.insert(player_id, msg.addr); 
                return Some((token, player_id, spawned)); 
            } 
        }
        let token = Uuid::new_v4().to_string(); 
        self.sessions.insert(msg.id, msg.addr);
        let mut player = Player::new(msg.id, token.clone(), "Guest".to_string(), 0.0, 0.0);
        player.quests = QuestSystem::get_initial_quests();
        self.world.players.insert(msg.id, player); 
        Some((token, msg.id, false))
    }
}

impl Handler<AcceptQuest> for GameEngine {
    type Result = ();
    fn handle(&mut self, msg: AcceptQuest, _: &mut Context<Self>) {
        if let Some(p) = self.world.players.get_mut(&msg.id) {
            if let Some(q) = p.quests.iter_mut().find(|q| q.id == msg.quest_id) {
                if q.state == QuestState::NotStarted {
                    q.state = QuestState::InProgress;
                    if let Some(addr) = self.sessions.get(&msg.id) {
                        addr.do_send(ServerMessage::QuestUpdate(q.clone()));
                        addr.do_send(ServerMessage::Notification { 
                            title: "Quest Accepted".to_string(), 
                            message: q.name.clone(), 
                            color: "#ff0".to_string() 
                        });
                    }
                }
            }
        }
    }
}

impl Handler<Spawn> for GameEngine {
    type Result = ();
    fn handle(&mut self, msg: Spawn, _: &mut Context<Self>) {
        if let Some(player) = self.world.players.get_mut(&msg.id) {
            let mut rng = rand::thread_rng();
            let (cx, cy, r) = (self.world.width / 2.0, self.world.height / 2.0, self.config.game.spawn_radius);
            let angle = rng.gen_range(0.0..std::f64::consts::TAU); let dist = rng.gen_range(0.0..r);
            player.x = cx + angle.cos() * dist;
            player.y = cy + angle.sin() * dist;
            player.health = 100.0;
            player.hunger = 100.0;
            player.cold = 50.0;
            player.spawned = true;
        }
    }
}

impl Handler<Leave> for GameEngine { type Result = (); fn handle(&mut self, msg: Leave, _: &mut Context<Self>) { self.sessions.remove(&msg.id); } }
impl Handler<Craft> for GameEngine { 
    type Result = (); 
    fn handle(&mut self, msg: Craft, _: &mut Context<Self>) { 
        if let Some(p) = self.world.players.get_mut(&msg.id) { 
            if !p.spawned { return; }
            CraftingSystem::craft(&mut p.inventory, msg.item); 
            p.stats.items_crafted += 1;
        }
        self.check_achievements(msg.id);
    } 
}
impl Handler<SelectSlot> for GameEngine { type Result = (); fn handle(&mut self, msg: SelectSlot, _: &mut Context<Self>) { if let Some(p) = self.world.players.get_mut(&msg.id) { if p.spawned && msg.slot < 7 { p.active_slot = msg.slot; } } } }
impl Handler<UpdateName> for GameEngine { type Result = (); fn handle(&mut self, msg: UpdateName, _: &mut Context<Self>) { if let Some(p) = self.world.players.get_mut(&msg.id) { let mut n = msg.name.trim().to_string(); if n.is_empty() { n = "Guest".to_string(); } if n.len() > 12 { n.truncate(12); } p.username = n; } } }
impl Handler<SwapSlots> for GameEngine { type Result = (); fn handle(&mut self, msg: SwapSlots, _: &mut Context<Self>) { if let Some(p) = self.world.players.get_mut(&msg.id) { if p.spawned && msg.from < p.inventory.slots.len() && msg.to < p.inventory.slots.len() { p.inventory.slots.swap(msg.from, msg.to); } } } }

impl Handler<Input> for GameEngine {
    type Result = ();
    fn handle(&mut self, msg: Input, _: &mut Context<Self>) {
        if let Some(p) = self.world.players.get(&msg.id) { if !p.spawned { return; } } else { return; }
        InteractionSystem::handle_movement(&mut self.world, msg.id, msg.dx, msg.dy);
        if msg.attack {
            let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64;
            let events = InteractionSystem::handle_attack(&mut self.world, &self.config, msg.id, now);
            for event in events {
                match event {
                    InteractionEvent::LevelUp { tool, level } => {
                        if let Some(addr) = self.sessions.get(&msg.id) {
                            addr.do_send(ServerMessage::Notification { 
                                title: "Tool Level Up!".to_string(), 
                                message: format!("{} reached level {}", tool, level), 
                                color: "#4f4".to_string() 
                            });
                        }
                    },
                    InteractionEvent::Gather { item, amount } => {
                        if let Some(p) = self.world.players.get_mut(&msg.id) {
                            let updated = QuestSystem::update_gather_progress(&mut p.quests, item, amount);
                            if let Some(addr) = self.sessions.get(&msg.id) {
                                for q in updated { addr.do_send(ServerMessage::QuestUpdate(q)); }
                            }
                        }
                    },
                    InteractionEvent::Kill { mob_type } => {
                        if let Some(p) = self.world.players.get_mut(&msg.id) {
                            let updated = QuestSystem::update_kill_progress(&mut p.quests, mob_type);
                            if let Some(addr) = self.sessions.get(&msg.id) {
                                for q in updated { addr.do_send(ServerMessage::QuestUpdate(q)); }
                            }
                        }
                    },
                    _ => {}
                }
            }
        }
        if msg.interact {
            if let Some(event) = InteractionSystem::handle_interact(&mut self.world, &self.config, msg.id) {
                match event {
                    InteractionEvent::Npc { npc_id } => {
                        if let Some(npc) = self.world.npcs.get(&npc_id) {
                            let mut text = match npc.n_type {
                                crate::game::entities::npc::NpcType::Elder => "Greetings, traveler. How can I help you?".to_string(),
                                crate::game::entities::npc::NpcType::Merchant => "Looking to trade?".to_string(),
                                crate::game::entities::npc::NpcType::Guard => "Move along, citizen.".to_string(),
                            };
                            let mut options = match npc.n_type {
                                crate::game::entities::npc::NpcType::Elder => vec!["Who are you?".to_string(), "I need a quest.".to_string(), "Goodbye".to_string()],
                                crate::game::entities::npc::NpcType::Merchant => vec!["Show me your wares.".to_string(), "Goodbye".to_string()],
                                crate::game::entities::npc::NpcType::Guard => vec!["Is it safe?".to_string(), "Goodbye".to_string()],
                            };

                            // Dynamic dialogue based on quest state
                            if let Some(p) = self.world.players.get(&msg.id) {
                                if npc.n_type == crate::game::entities::npc::NpcType::Elder {
                                    // Check Wood Gatherer
                                    let wood_q = p.quests.iter().find(|q| q.id == "wood_gatherer");
                                    let wolf_q = p.quests.iter().find(|q| q.id == "wolf_hunter");

                                    if let Some(q) = wood_q {
                                        if q.state == QuestState::InProgress {
                                            text = "How is that wood gathering going?".to_string();
                                            options = vec!["Still working on it.".to_string(), "Goodbye".to_string()];
                                        } else if q.state == QuestState::ReadyToTurnIn {
                                            text = "Ah, I see you have the wood! Thank you.".to_string();
                                            options = vec!["Complete 'Wood Gatherer'".to_string(), "Goodbye".to_string()];
                                        } else if q.state == QuestState::Completed {
                                            if let Some(wq) = wolf_q {
                                                match wq.state {
                                                    QuestState::NotStarted => {
                                                        text = "The wolves are acting up. Can you help?".to_string();
                                                        options = vec!["Tell me more.".to_string(), "Goodbye".to_string()];
                                                    },
                                                    QuestState::InProgress => {
                                                        text = "Have you dealt with those wolves yet?".to_string();
                                                        options = vec!["Working on it.".to_string(), "Goodbye".to_string()];
                                                    },
                                                    QuestState::ReadyToTurnIn => {
                                                        text = "The village feels safer already. Thank you!".to_string();
                                                        options = vec!["Complete 'Wolf Hunter'".to_string(), "Goodbye".to_string()];
                                                    },
                                                    QuestState::Completed => {
                                                        text = "You are a true hero of the village.".to_string();
                                                        options = vec!["Happy to help.".to_string(), "Goodbye".to_string()];
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            if let Some(addr) = self.sessions.get(&msg.id) {
                                addr.do_send(ServerMessage::NpcInteraction {
                                    npc_id,
                                    npc_type: npc.n_type.clone(),
                                    name: npc.name.clone(),
                                    text,
                                    options,
                                    trade_items: npc.trade_inventory.clone(),
                                });
                            }
                        }
                    },
                    _ => {}
                }
            }
        }
        self.check_achievements(msg.id);
    }
}

impl Handler<NpcAction> for GameEngine {
    type Result = ();
    fn handle(&mut self, msg: NpcAction, _: &mut Context<Self>) {
        if let Some(npc) = self.world.npcs.get(&msg.npc_id) {
            let mut response = match (npc.n_type.clone(), msg.option_index) {
                (crate::game::entities::npc::NpcType::Elder, 0) => "I am the Elder of this village. I have seen many seasons.".to_string(),
                (crate::game::entities::npc::NpcType::Elder, 1) => "Go gather 10 wood and I shall reward you.".to_string(),
                (crate::game::entities::npc::NpcType::Merchant, 0) => "I have the finest goods in the land.".to_string(),
                _ => "Goodbye.".to_string(),
            };
            let mut options = vec!["Okay".to_string()];

            // Logic for quest actions
            if let Some(p) = self.world.players.get_mut(&msg.id) {
                if npc.n_type == crate::game::entities::npc::NpcType::Elder {
                    let wood_q_idx = p.quests.iter().position(|q| q.id == "wood_gatherer");
                    let wolf_q_idx = p.quests.iter().position(|q| q.id == "wolf_hunter");

                    if let Some(idx) = wood_q_idx {
                        let q = &mut p.quests[idx];
                        if msg.option_index == 1 && q.state == QuestState::NotStarted {
                            q.state = QuestState::InProgress;
                            response = "Excellent. Return to me when you have 10 pieces of wood.".to_string();
                            if let Some(addr) = self.sessions.get(&msg.id) {
                                addr.do_send(ServerMessage::QuestUpdate(q.clone()));
                                addr.do_send(ServerMessage::Notification { 
                                    title: "Quest Accepted".to_string(), 
                                    message: q.name.clone(), 
                                    color: "#ff0".to_string() 
                                });
                            }
                        } else if q.state == QuestState::ReadyToTurnIn && msg.option_index == 0 {
                            q.state = QuestState::Completed;
                            response = "Wonderful! Here is your reward (XP and gratitude).".to_string();
                            // Consume items for gathering quests
                            for obj in q.objectives.iter() {
                                if let crate::game::systems::quests::ObjectiveType::Gather { item, count, .. } = obj {
                                    let mut remaining = *count;
                                    for slot in p.inventory.slots.iter_mut() {
                                        if let Some(inv_item) = slot {
                                            if inv_item.kind == *item {
                                                let to_remove = inv_item.amount.min(remaining);
                                                inv_item.amount -= to_remove;
                                                remaining -= to_remove;
                                                if inv_item.amount == 0 { *slot = None; }
                                                if remaining == 0 { break; }
                                            }
                                        }
                                    }
                                }
                            }
                            if let Some(addr) = self.sessions.get(&msg.id) {
                                addr.do_send(ServerMessage::QuestUpdate(q.clone()));
                                addr.do_send(ServerMessage::Notification { 
                                    title: "Quest Completed!".to_string(), 
                                    message: q.name.clone(), 
                                    color: "#0f0".to_string() 
                                });
                            }
                        }
                    }

                    let wood_q_done = p.quests.iter().any(|q| q.id == "wood_gatherer" && q.state == QuestState::Completed);
                    if let Some(idx) = wolf_q_idx {
                        let q = &mut p.quests[idx];
                        if wood_q_done && q.state == QuestState::NotStarted && msg.option_index == 1 {
                             q.state = QuestState::InProgress;
                             response = "Thank you. Kill 3 Wolves and return.".to_string();
                             if let Some(addr) = self.sessions.get(&msg.id) {
                                addr.do_send(ServerMessage::QuestUpdate(q.clone()));
                                addr.do_send(ServerMessage::Notification { 
                                    title: "Quest Accepted".to_string(), 
                                    message: q.name.clone(), 
                                    color: "#ff0".to_string() 
                                });
                            }
                        } else if q.state == QuestState::ReadyToTurnIn && msg.option_index == 0 {
                             q.state = QuestState::Completed;
                             response = "The wolves are gone! You've saved us.".to_string();
                             if let Some(addr) = self.sessions.get(&msg.id) {
                                addr.do_send(ServerMessage::QuestUpdate(q.clone()));
                                addr.do_send(ServerMessage::Notification { 
                                    title: "Quest Completed!".to_string(), 
                                    message: q.name.clone(), 
                                    color: "#0f0".to_string() 
                                });
                            }
                        }
                    }
                }
            }

            if let Some(addr) = self.sessions.get(&msg.id) {
                addr.do_send(ServerMessage::NpcInteraction {
                    npc_id: msg.npc_id,
                    npc_type: npc.n_type.clone(),
                    name: npc.name.clone(),
                    text: response,
                    options,
                    trade_items: npc.trade_inventory.clone(),
                });
            }
        }
    }
}

impl Handler<Trade> for GameEngine {
    type Result = ();
    fn handle(&mut self, msg: Trade, _: &mut Context<Self>) {
        // Implementation for trading logic would go here
        // For now, it's a placeholder to satisfy the MMORPG requirement
    }
}

