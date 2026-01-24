use std::collections::HashMap;
use crate::net::{Message, MessageType, InputData, SnapshotData};

#[derive(Debug, Clone)]
pub struct Player {
    pub id: String,
    pub position: (f32, f32),
    pub health: f32,
    pub inventory: HashMap<u32, u32>,  // item_id -> quantity
}

#[derive(Debug)]
pub struct WorldState {
    pub players: HashMap<String, Player>,
    pub tick: u64,
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            tick: 0,
        }
    }

    pub fn process_input(&mut self, input: &crate::net::InputEvent) {
        // Add player if new
        self.players.entry(input.session_id.clone()).or_insert(Player {
            id: input.session_id.clone(),
            position: (0.0, 0.0),
            health: 100.0,
            inventory: HashMap::new(),
        });

        // Process message
        match &input.message.msg_type {
            MessageType::Input(data) => {
                if let Some(player) = self.players.get_mut(&input.session_id) {
                    match data.action.as_str() {
                        "move" => {
                            // Simple move: assume data contains dx, dy
                            // In real, decode from payload
                            player.position.0 += 1.0;  // Placeholder
                            player.position.1 += 1.0;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    pub fn tick(&mut self) {
        self.tick += 1;
        // Placeholder: decrease health over time
        for player in self.players.values_mut() {
            player.health -= 0.1;
            if player.health < 0.0 {
                player.health = 0.0;
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
}