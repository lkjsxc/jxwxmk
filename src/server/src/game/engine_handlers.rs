use actix::{ActorFutureExt, Handler};
use actix::fut::wrap_future;

use crate::protocol::ServerMessage;
use crate::persistence::save_player;

use crate::game::engine_messages::{EngineEventCommand, JoinCommand, JoinResult, LeaveCommand, RevokeSession};
use crate::game::engine_tick::enqueue_event;
use crate::game::entities::PlayerState;
use crate::game::GameEngine;
use crate::persistence::load_player_by_id;

impl Handler<JoinCommand> for GameEngine {
    type Result = actix::ResponseActFuture<Self, JoinResult>;

    fn handle(&mut self, msg: JoinCommand, _ctx: &mut Self::Context) -> Self::Result {
        let player_id = msg.player_id;
        let token = msg.token;
        let session = msg.session;

        if let Some(existing) = self.sessions.get(&player_id) {
            let _ = existing.do_send(ServerMessage::SessionRevoked {
                reason: "login_elsewhere".to_string(),
            });
        }
        self.sessions.insert(player_id, session);

        let db = self.db.clone();
        let config = self.config.clone();
        let query_config = config.clone();
        let fut = wrap_future(async move { load_player_by_id(&db, player_id, &query_config).await })
            .map(move |player_opt, act: &mut GameEngine, _ctx| {
                let mut player = player_opt.unwrap_or_else(|| {
                    PlayerState::new(
                        player_id,
                        token,
                        config.balance.player.inventory_slots,
                        config.balance.player.max_health,
                    )
                });
                player.token = token;
                act.world.upsert_player(player.clone());
                JoinResult {
                    id: player.id,
                    token: player.token,
                    spawned: player.spawned,
                }
            });

        Box::pin(fut)
    }
}

impl Handler<LeaveCommand> for GameEngine {
    type Result = ();

    fn handle(&mut self, msg: LeaveCommand, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(player) = self.world.get_player_mut(&msg.player_id) {
            player.spawned = false;
        }
        self.sessions.remove(&msg.player_id);
        let pool = self.db.clone();
        if let Some(player) = self.world.get_player(&msg.player_id) {
            let player = player.clone();
            actix::spawn(async move {
                let _ = save_player(&pool, &player).await;
            });
        }
    }
}

impl Handler<EngineEventCommand> for GameEngine {
    type Result = ();

    fn handle(&mut self, msg: EngineEventCommand, _ctx: &mut Self::Context) -> Self::Result {
        enqueue_event(self, msg.event);
    }
}

impl Handler<RevokeSession> for GameEngine {
    type Result = ();

    fn handle(&mut self, msg: RevokeSession, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(session) = self.sessions.remove(&msg.player_id) {
            let _ = session.do_send(ServerMessage::SessionRevoked { reason: msg.reason });
        }
    }
}
