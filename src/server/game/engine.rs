use actix::prelude::*;
use super::world::World;
use super::messages::{ClientConnected, ClientDisconnected, ClientRequest};
use std::time::Duration;
use log::info;

pub struct GameEngine {
    world: World,
}

impl GameEngine {
    pub fn new() -> Self {
        Self {
            world: World::new(),
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
        // 20Hz tick rate -> 50ms
        ctx.run_interval(Duration::from_millis(50), |act, ctx| {
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

    #[actix::test]
    async fn test_engine_startup() {
        let engine = GameEngine::new().start();
        assert!(engine.connected());
    }
}
