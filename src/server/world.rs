use std::collections::HashMap;
use crate::net::{Message, MessageType, InputData, SnapshotData};

mod gathering;
mod crafting;
mod survival;
mod combat;

#[derive(Debug, Clone)]
pub struct Player {
    pub id: String,
    pub position: (f32, f32),
    pub health: f32,
    pub hunger: f32,
    pub inventory: HashMap<u32, u32>,  // item_id -> quantity
}

#[derive(Debug)]
pub struct ResourceNode {
    pub id: u32,
    pub node_type: String,  // "tree", "rock"
    pub position: (f32, f32),
    pub depleted: bool,
    pub respawn_tick: u64,
}

#[derive(Debug)]
pub struct WorldState {
    pub players: HashMap<String, Player>,
    pub resources: Vec<ResourceNode>,
    pub tick: u64,
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            resources: vec![
                ResourceNode { id: 1, node_type: "tree".to_string(), position: (100.0, 200.0), depleted: false, respawn_tick: 0 },
                ResourceNode { id: 2, node_type: "rock".to_string(), position: (150.0, 250.0), depleted: false, respawn_tick: 0 },
            ],
            tick: 0,
        }
    }

    pub fn process_input(&mut self, input: &crate::net::InputEvent) {
        // Validate session (placeholder)
        if !self.is_valid_session(&input.session_id) {
            return;
        }
        // Add player if new
        self.players.entry(input.session_id.clone()).or_insert(Player {
            id: input.session_id.clone(),
            position: (0.0, 0.0),
            health: 100.0,
            hunger: 100.0,
            inventory: HashMap::new(),
        });

        // Process message
        match &input.message.msg_type {
            MessageType::Input(data) => {
                if let Some(player) = self.players.get_mut(&input.session_id) {
                    match data.action.as_str() {
                        "move" => {
                            player.position.0 += 1.0;
                            player.position.1 += 1.0;
                        }
                        "gather" => {
                            gathering::gather_resource(player, &mut self.resources, data.data.clone());
                        }
                        "craft" => {
                            crafting::craft_item(player, data.data.clone());
                        }
                        "eat" => {
                            survival::consume_food(player, data.data.clone());
                        }
                        "attack" => {
                            // Placeholder for PvP
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    pub fn tick(&mut self, pool: &sqlx::PgPool) {
        self.tick += 1;
        for player in self.players.values_mut() {
            survival::update_survival(player);
            if player.health <= 0.0 {
                // Respawn or end game
            }
        }
        gathering::update_resources(&mut self.resources, self.tick);
        if self.tick % 100 == 0 {  // Checkpoint every 100 ticks
            self.checkpoint(pool);
        }
    }

    pub async fn checkpoint(&self, pool: &sqlx::PgPool) {
        for player in self.players.values() {
            if let Err(e) = crate::db::save_player(pool, player).await {
                eprintln!("Save error: {}", e);
            }
        }
    }

    pub fn get_sessions(&self) -> Vec<String> {
        self.players.keys().cloned().collect()
    }

    pub fn create_snapshot(&self, server_tick: u64) -> Message {
        let snapshot = SnapshotData {
            server_tick,
            world_state: vec![],  // Placeholder: serialize world
        };
        Message {
            protocol_version: 1,
            msg_type: MessageType::Snapshot(snapshot),
            seq: 0,
            payload: vec![],
        }
    }

    pub fn is_valid_session(&self, session_id: &str) -> bool {
        // Placeholder: check against DB or cache
        session_id.starts_with("session_")
    }
}