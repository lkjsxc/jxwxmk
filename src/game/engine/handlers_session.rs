use actix::prelude::*;
use uuid::Uuid;

use crate::protocol::server::ServerMessage;
use crate::server::database;

use super::{messages::*, GameEngine, SessionHandle};

impl Handler<Join> for GameEngine {
    type Result = ResponseActFuture<Self, JoinResult>;

    fn handle(&mut self, msg: Join, _ctx: &mut Self::Context) -> Self::Result {
        let config = self.config.clone();
        let db = self.db.clone();
        let session_id = msg.session_id;
        let addr = msg.addr;
        let join_future = async move {
            let (player_id, token, player_state) = if let Some(token) = msg.token {
                if let Some(record) = database::load_player_by_token(&db, token).await.ok().flatten() {
                    (record.id, record.token, record.into_player_state(&config))
                } else {
                    let id = Uuid::new_v4();
                    let token = Uuid::new_v4();
                    let player = crate::game::world::entities::PlayerState::new(
                        id,
                        token,
                        config.balance.player.inventory_slots,
                    );
                    database::upsert_player(&db, &player).await.ok();
                    (id, token, player)
                }
            } else {
                let id = Uuid::new_v4();
                let token = Uuid::new_v4();
                let player = crate::game::world::entities::PlayerState::new(
                    id,
                    token,
                    config.balance.player.inventory_slots,
                );
                database::upsert_player(&db, &player).await.ok();
                (id, token, player)
            };

            (player_id, token, player_state)
        };

        Box::pin(join_future.into_actor(self).map(move |result, actor, _ctx| {
            let (player_id, token, player_state) = result;
            let spawned = player_state.spawned;
            actor.world.players.insert(player_id, player_state);

            if let Some(existing_session) = actor.player_sessions.get(&player_id).cloned() {
                if let Some(existing) = actor.sessions.get(&existing_session) {
                    let _ = existing.addr.do_send(ServerMessage::SessionRevoked {
                        reason: "login_elsewhere".to_string(),
                    });
                }
                actor.sessions.remove(&existing_session);
            }
            actor.sessions.insert(
                session_id,
                SessionHandle {
                    player_id,
                    addr,
                },
            );
            actor.player_sessions.insert(player_id, session_id);

            JoinResult {
                player_id,
                token,
                spawned,
            }
        }))
    }
}

impl Handler<Leave> for GameEngine {
    type Result = ();

    fn handle(&mut self, msg: Leave, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(session) = self.sessions.remove(&msg.session_id) {
            self.player_sessions.remove(&session.player_id);
            if let Some(player) = self.world.players.get_mut(&session.player_id) {
                player.spawned = false;
                let player = player.clone();
                let db = self.db.clone();
                actix::spawn(async move {
                    let _ = database::upsert_player(&db, &player).await;
                });
            }
        }
    }
}

impl Handler<RevokePlayer> for GameEngine {
    type Result = ();

    fn handle(&mut self, msg: RevokePlayer, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(session_id) = self.player_sessions.remove(&msg.player_id) {
            if let Some(session) = self.sessions.remove(&session_id) {
                let _ = session.addr.do_send(ServerMessage::SessionRevoked {
                    reason: "login_elsewhere".to_string(),
                });
            }
        }
    }
}
