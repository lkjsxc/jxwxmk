use actix::prelude::*;
use uuid::Uuid;

use crate::protocol::server::ServerMessage;
use crate::server::database;

use super::GameEngine;

impl GameEngine {
    pub(crate) fn send_to_player(&self, player_id: Uuid, message: ServerMessage) {
        if let Some(session_id) = self.player_sessions.get(&player_id) {
            if let Some(session) = self.sessions.get(session_id) {
                let _ = session.addr.do_send(message);
            }
        }
    }

    pub(crate) fn persist_players_if_needed(&self) {
        let save_interval = (self.config.server.tick_rate * 10.0).max(1.0) as u64;
        if self.tick_counter % save_interval != 0 {
            return;
        }
        let players: Vec<_> = self.world.players.values().cloned().collect();
        let db = self.db.clone();
        actix::spawn(async move {
            for player in players {
                let _ = database::upsert_player(&db, &player).await;
            }
        });
    }
}
