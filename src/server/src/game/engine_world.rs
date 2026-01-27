use std::collections::HashSet;

use crate::config::Config;
use crate::protocol::{ChunkRemoveData, EntityDeltaData, ServerMessage};

use crate::game::chunk::ChunkCoord;
use crate::game::delta::{build_chunk_add, snapshot_barrier, snapshot_player};
use crate::game::entities::BarrierCore;
use crate::game::messages::OutboundMessage;
use crate::game::world::{Settlement, World};
use crate::game::world_gen::generate_chunk;

pub fn initialize_settlements(world: &mut World, config: &Config) {
    let name = config
        .settlements
        .settlement_names
        .first()
        .cloned()
        .unwrap_or_else(|| "Elder Hollow".to_string());
    let core = BarrierCore {
        id: "core_0".to_string(),
        level: config.settlements.default_core_level,
        base_range: config.settlements.core_base_range,
        level_multiplier: config.settlements.core_level_multiplier,
        faction: "neutral".to_string(),
        integrity: 100.0,
        x: 0.0,
        y: 0.0,
    };
    let settlement = Settlement {
        id: "settlement_0".to_string(),
        name: name.clone(),
        core,
        spawn_x: 0.0,
        spawn_y: 0.0,
    };
    world.settlements.insert(settlement.id.clone(), settlement);
}

pub fn ensure_chunk(world: &mut World, config: &Config, coord: ChunkCoord, tick: u64) {
    if world.chunks.contains_key(&coord) {
        return;
    }
    let chunk = generate_chunk(world.seed, coord, config, tick);
    world.chunks.insert(coord, chunk);
}

pub fn update_interest_sets(
    world: &mut World,
    config: &Config,
    tick: u64,
    outbox: &mut Vec<OutboundMessage>,
) -> HashSet<ChunkCoord> {
    let mut active_chunks = HashSet::new();
    let view_radius = config.world.view_radius;
    let sim_radius = config.world.simulation_radius;

    let snapshots: Vec<_> = world
        .players
        .iter()
        .map(|(id, player)| (*id, player.spawned, player.x, player.y))
        .collect();

    for (player_id, spawned, x, y) in snapshots {
        if !spawned {
            continue;
        }
        let coord = world.chunk_coord_from_pos(x, y, config.world.chunk_size);

        let mut new_interest = HashSet::new();
        for dx in -view_radius..=view_radius {
            for dy in -view_radius..=view_radius {
                let target = ChunkCoord::new(coord.cx + dx, coord.cy + dy);
                ensure_chunk(world, config, target, tick);
                new_interest.insert(target);
            }
        }

        for dx in -sim_radius..=sim_radius {
            for dy in -sim_radius..=sim_radius {
                active_chunks.insert(ChunkCoord::new(coord.cx + dx, coord.cy + dy));
            }
        }

        if let Some(player) = world.players.get_mut(&player_id) {
            player.chunk = (coord.cx, coord.cy);
        }

        let previous = world.interest_sets.insert(player_id, new_interest.clone());
        let previous = previous.unwrap_or_default();

        for coord in new_interest.difference(&previous) {
            if let Some(chunk) = world.chunks.get(coord) {
                let data = build_chunk_add(chunk);
                outbox.push((player_id, ServerMessage::ChunkAdd { data }));
            }
        }

        for coord in previous.difference(&new_interest) {
            let data = ChunkRemoveData {
                coord: [coord.cx, coord.cy],
            };
            outbox.push((player_id, ServerMessage::ChunkRemove { data }));
        }
    }

    active_chunks
}

pub fn build_entity_deltas(world: &World, config: &Config, outbox: &mut Vec<OutboundMessage>) {
    for (player_id, interest) in &world.interest_sets {
        for coord in interest {
            if let Some(chunk) = world.chunks.get(coord) {
                let mut updates = Vec::new();
                for resource in chunk.resources.values() {
                    updates.push(crate::game::delta::snapshot_resource(resource));
                }
                for mob in chunk.mobs.values() {
                    updates.push(crate::game::delta::snapshot_mob(mob));
                }
                for structure in chunk.structures.values() {
                    updates.push(crate::game::delta::snapshot_structure(structure));
                }
                for npc in chunk.npcs.values() {
                    updates.push(crate::game::delta::snapshot_npc(npc));
                }
                for player in world.players.values() {
                    if player.spawned && player.chunk == (coord.cx, coord.cy) {
                        updates.push(snapshot_player(player));
                    }
                }
                for settlement in world.settlements.values() {
                    let settlement_coord = world.chunk_coord_from_pos(
                        settlement.core.x,
                        settlement.core.y,
                        config.world.chunk_size,
                    );
                    if settlement_coord == *coord {
                        updates.push(snapshot_barrier(&settlement.core));
                    }
                }

                let data = EntityDeltaData {
                    chunk: [coord.cx, coord.cy],
                    updates,
                    removes: Vec::new(),
                };
                outbox.push((*player_id, ServerMessage::EntityDelta { data }));
            }
        }
    }
}
