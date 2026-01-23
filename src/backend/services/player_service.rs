use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::models::player::Player;

pub struct PlayerService {
    players: Arc<RwLock<HashMap<Uuid, Player>>>,
}

impl PlayerService {
    pub fn new() -> Self {
        Self {
            players: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_player(&self, player: Player) {
        let mut players = self.players.write().await;
        players.insert(player.id, player);
    }

    pub async fn get_player(&self, player_id: &Uuid) -> Option<Player> {
        let players = self.players.read().await;
        players.get(player_id).cloned()
    }

    pub async fn update_player_position(&self, player_id: Uuid, x: f32, y: f32) -> bool {
        let mut players = self.players.write().await;
        if let Some(player) = players.get_mut(&player_id) {
            player.position.x = x;
            player.position.y = y;
            true
        } else {
            false
        }
    }
}