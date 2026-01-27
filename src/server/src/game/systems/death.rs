use crate::protocol::{NotificationData, ServerMessage};

use crate::game::messages::OutboundMessage;
use crate::game::world::World;

pub fn handle_deaths(world: &mut World, outbox: &mut Vec<OutboundMessage>) {
    for player in world.players.values_mut() {
        if player.spawned && player.health <= 0.0 {
            player.spawned = false;
            player.stats.deaths = player.stats.deaths.saturating_add(1);
            let data = NotificationData {
                text: "You died.".to_string(),
            };
            outbox.push((player.id, ServerMessage::Notification { data }));
        }
    }
}
