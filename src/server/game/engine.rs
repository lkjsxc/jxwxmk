use actix::prelude::*;
use super::world::World;
use super::messages::{ClientConnected, ClientDisconnected, ClientRequest};
use std::time::Duration;
use log::info;
use crate::config::GameConfig;

pub struct GameEngine {
    world: World,
    config: GameConfig,
}

impl GameEngine {
    pub fn new(config: GameConfig) -> Self {
        Self {
            world: World::new(),
            config,
        }
    }

    fn tick(&mut self, ctx: &mut Context<Self>) {
        // TODO: Implement tick loop logic
        // 1. Systems update
        // 2. Broadcast updates
    }
}

impl Actor for GameEngine {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("GameEngine started");
        // Use configured tick rate
        let tick_ms = 1000 / self.config.server.tick_rate;
        ctx.run_interval(Duration::from_millis(tick_ms), |act, ctx| {
            act.tick(ctx);
        });
    }
}

impl Handler<ClientConnected> for GameEngine {
    type Result = ();

    fn handle(&mut self, msg: ClientConnected, _ctx: &mut Context<Self>) {
        info!("Client connected: {}", msg.id);
        // TODO: Add to session registry / player map
    }
}

impl Handler<ClientDisconnected> for GameEngine {
    type Result = ();

    fn handle(&mut self, msg: ClientDisconnected, _ctx: &mut Context<Self>) {
        info!("Client disconnected: {}", msg.id);
        // TODO: Mark offline / remove session
    }
}

impl Handler<ClientRequest> for GameEngine {
    type Result = ();

    fn handle(&mut self, msg: ClientRequest, _ctx: &mut Context<Self>) {
        // info!("Received request from {}: {:?}", msg.id, msg.msg);
        // TODO: Process input (enqueue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{ServerConfig, WorldConfig};

    #[actix::test]
    async fn test_engine_startup() {
        let config = GameConfig {
            server: ServerConfig { port: 8080, tick_rate: 20 },
            world: WorldConfig { seed: 12345, chunk_size: 16 },
        };
        let engine = GameEngine::new(config).start();
        assert!(engine.connected());
    }
}
