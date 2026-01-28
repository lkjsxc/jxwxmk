use world::{World, ChunkCoord, Entity};
use uuid::Uuid;
use std::collections::{HashMap, HashSet};
use actix::Recipient;
use protocol::{ServerMessage, EntitySnapshot};
use crate::engine::OutboundMessage;

pub fn update_interest(
    world: &mut World,
    sessions: &HashMap<Uuid, Recipient<OutboundMessage>>,
) {
    let view_radius = world.config.world.view_radius_chunks as i32;
    let sim_radius = world.config.world.sim_radius_chunks as i32;
    let chunk_size = world.config.world.chunk_size_wu as f32;
    let seed = world.config.world.seed.clone();
    let config = world.config.clone();

    let player_ids: Vec<Uuid> = world.players.keys().cloned().collect();
    let mut all_desired_view = HashSet::new();
    let mut all_desired_sim = HashSet::new();

    for pid in player_ids {
        let mut adds = Vec::new();
        let mut removes = Vec::new();

        if let Some(player) = world.players.get_mut(&pid) {
            let cx = (player.pos.x / chunk_size).floor() as i32;
            let cy = (player.pos.y / chunk_size).floor() as i32;
            player.chunk = ChunkCoord { x: cx, y: cy };

            let mut desired_view = HashSet::new();
            for dx in -view_radius..=view_radius {
                for dy in -view_radius..=view_radius {
                    let c = ChunkCoord { x: cx + dx, y: cy + dy };
                    desired_view.insert(c);
                    all_desired_view.insert(c);
                }
            }

            for dx in -sim_radius..=sim_radius {
                for dy in -sim_radius..=sim_radius {
                    all_desired_sim.insert(ChunkCoord { x: cx + dx, y: cy + dy });
                }
            }

            for c in &desired_view {
                if !player.active_view.contains(c) { adds.push(*c); }
            }
            for c in &player.active_view {
                if !desired_view.contains(c) { removes.push(*c); }
            }
            player.active_view = desired_view;
        }

        for coord in adds {
            if !world.chunks.contains_key(&coord) {
                let chunk = world::gen::generate_chunk(coord, &seed, &config);
                world.chunks.insert(coord, chunk);
            }
            if let Some(recipient) = sessions.get(&pid) {
                let chunk = world.chunks.get(&coord).unwrap();
                let msg = ServerMessage::ChunkAdd {
                    data: protocol::ChunkAddData {
                        coord: [coord.x, coord.y],
                        biome: chunk.biome.clone(),
                        entities: protocol::ChunkEntities {
                            resources: chunk.resources.iter().map(|(id, e)| (id.to_string(), map_entity(e))).collect(),
                            mobs: chunk.mobs.iter().map(|(id, e)| (id.to_string(), map_entity(e))).collect(),
                            structures: chunk.structures.iter().map(|(id, e)| (id.to_string(), map_entity(e))).collect(),
                            npcs: chunk.npcs.iter().map(|(id, e)| (id.to_string(), map_entity(e))).collect(),
                        }
                    }
                };
                if let Ok(json) = serde_json::to_string(&msg) { 
                    let _ = recipient.do_send(OutboundMessage(json)); 
                }
            }
        }

        for coord in removes {
            if let Some(recipient) = sessions.get(&pid) {
                let msg = ServerMessage::ChunkRemove {
                    data: protocol::ChunkRemoveData { coord: [coord.x, coord.y] }
                };
                if let Ok(json) = serde_json::to_string(&msg) { 
                    let _ = recipient.do_send(OutboundMessage(json)); 
                }
            }
        }
    }

    // Manage active (simulated) chunks
    world.active_chunks = all_desired_sim;

    // Optional: Unload chunks that are neither in view nor in sim radius of any player
    // For now, keep them in memory but they won't tick if not in world.active_chunks.
}

fn map_entity(e: &Entity) -> EntitySnapshot {
    EntitySnapshot {
        id: e.id.to_string(),
        kind: match e.kind {
            world::EntityKind::Player => "player".into(),
            world::EntityKind::Resource => "resource".into(),
            world::EntityKind::Mob => "mob".into(),
            world::EntityKind::Structure => "structure".into(),
            world::EntityKind::Npc => "npc".into(),
        },
        subtype: e.subtype.clone(),
        x: e.pos.x,
        y: e.pos.y,
        hp: Some(e.hp),
        max_hp: Some(e.max_hp),
        hunger: None,
        temp: None,
        level: None,
        name: e.name.clone(),
        range: None,
    }
}