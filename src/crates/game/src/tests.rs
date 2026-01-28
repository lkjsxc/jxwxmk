#[cfg(test)]
mod tests {
    use crate::GameEngine;
    use config::Config;
    use crate::events::GameEvent;
    use std::time::Duration;
    use uuid::Uuid;
    use actix::prelude::*;
    use crate::engine::OutboundMessage;

    struct DummyRecipient;
    impl Actor for DummyRecipient { type Context = Context<Self>; }
    impl Handler<OutboundMessage> for DummyRecipient {
        type Result = ();
        fn handle(&mut self, _msg: OutboundMessage, _ctx: &mut Self::Context) {}
    }

    #[actix_rt::test]
    async fn test_tick_processes_events() {
        let config = Config::default();
        let mut engine = GameEngine::new(config, None);
        
        let dummy = DummyRecipient.start();
        let recipient = dummy.recipient();

        let player_id = Uuid::new_v4();
        let event = GameEvent::PlayerJoin {
            player_id,
            name: "Tester".into(),
            token: Uuid::new_v4(),
            recipient,
        };
        
        engine.enqueue(event);
        assert_eq!(engine.event_queue.len(), 1);
        
        engine.tick(Duration::from_millis(50));
        
        assert_eq!(engine.event_queue.len(), 0);
        assert!(engine.world.players.contains_key(&player_id));
        assert_eq!(engine.tick_count, 1);
    }
}