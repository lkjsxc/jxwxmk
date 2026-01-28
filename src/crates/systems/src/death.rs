use world::{World, PlayerState, ChunkCoord, Vec2};
use std::time::Duration;
use uuid::Uuid;

pub fn tick(world: &mut World, _dt: Duration) {
    let mut dead_players = Vec::new();
    for player in world.players.values() {
        if player.spawned && player.hp <= 0.0 {
            dead_players.push(player.id);
        }
    }

    for pid in dead_players {
        handle_death(world, pid);
    }
}

fn handle_death(world: &mut World, player_id: Uuid) {
    if let Some(player) = world.players.get_mut(&player_id) {
        player.spawned = false;
        
        // Drop logic stub
        // For MVP: just clear inventory
        for slot in player.inventory.iter_mut() {
            *slot = None;
        }

        // Reset vitals for next spawn
        player.hp = player.max_hp;
        player.hunger = 100.0;
        player.temp = 50.0;
        
        world.pending_notifications.push((player_id, "You died!".to_string()));
        println!("Player {} died", player_id);
    }
}

pub fn respawn(world: &mut World, player_id: Uuid) {
    if let Some(player) = world.players.get_mut(&player_id) {
        if !player.spawned {
            // Respawn at origin village (or bound settlement)
            player.pos = Vec2 { x: 64.0, y: 64.0 };
            player.chunk = ChunkCoord { x: 0, y: 0 };
            player.spawned = true;
            println!("Player {} respawned", player_id);
        }
    }
}
