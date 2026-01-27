use crate::config::Config;
use crate::protocol::{NpcInteractionData, ServerMessage};

use crate::game::chunk::{ChunkCoord, RespawnEntry, RespawnKind};
use crate::game::entities::{ItemId, PlayerId};
use crate::game::messages::OutboundMessage;
use crate::game::world::World;

pub const ACTION_RANGE: f32 = 2.0;

pub fn try_gather(world: &mut World, config: &Config, player_id: PlayerId) -> bool {
    let (player_x, player_y, chunk_pos) = match world.get_player(&player_id) {
        Some(player) => (player.x, player.y, player.chunk),
        None => return false,
    };
    let coord = ChunkCoord::new(chunk_pos.0, chunk_pos.1);
    let gather_bonus = stat_bonus(world, &player_id, "gather");

    let mut drop_item = None;
    let mut gathered = false;
    let mut acted = false;

    let Some(chunk) = world.chunks.get_mut(&coord) else {
        return false;
    };

    let mut best_id = None;
    let mut best_dist = f32::MAX;
    for (id, resource) in chunk.resources.iter() {
        let dx = resource.x - player_x;
        let dy = resource.y - player_y;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist < best_dist {
            best_dist = dist;
            best_id = Some(id.clone());
        }
    }

    let Some(target_id) = best_id else {
        return false;
    };
    if best_dist > ACTION_RANGE {
        return false;
    }

    if let Some(resource) = chunk.resources.get_mut(&target_id) {
        let damage = config.balance.tools.base_gather_damage + gather_bonus;
        resource.amount -= damage;
        if resource.amount <= 0.0 {
            drop_item = Some(match resource.r_type.as_str() {
                "tree" => ItemId::new("Wood"),
                "rock" => ItemId::new("Stone"),
                _ => ItemId::new("Berry"),
            });
            let respawn = RespawnEntry {
                kind: RespawnKind::Resource {
                    r_type: resource.r_type.clone(),
                    level: resource.level,
                },
                remaining: config.spawning.resource_respawn_seconds,
            };
            chunk.resources.remove(&target_id);
            chunk.cooldowns.push(respawn);
            gathered = true;
        }
        acted = true;
    }

    if let Some(item) = drop_item {
        if let Some(player) = world.get_player_mut(&player_id) {
            let _ = player.inventory.add_item(item, 1);
            if gathered {
                player.stats.gathers = player.stats.gathers.saturating_add(1);
            }
        }
    }

    acted
}

pub fn try_attack(world: &mut World, config: &Config, player_id: PlayerId) {
    let (player_x, player_y, chunk_pos) = match world.get_player(&player_id) {
        Some(player) => (player.x, player.y, player.chunk),
        None => return,
    };
    let coord = ChunkCoord::new(chunk_pos.0, chunk_pos.1);
    let damage_bonus = stat_bonus(world, &player_id, "damage");
    let base_damage = config.balance.combat.base_melee_damage + damage_bonus;

    let mut attacked_mob = false;
    let mut mob_killed = false;

    if let Some(chunk) = world.chunks.get_mut(&coord) {
        let mut best_id = None;
        let mut best_dist = f32::MAX;
        for (id, mob) in chunk.mobs.iter() {
            let dx = mob.x - player_x;
            let dy = mob.y - player_y;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist < best_dist {
                best_dist = dist;
                best_id = Some(id.clone());
            }
        }

        if let Some(mob_id) = best_id {
            if best_dist <= ACTION_RANGE {
                attacked_mob = true;
                if let Some(mob) = chunk.mobs.get_mut(&mob_id) {
                    mob.health -= base_damage;
                    if mob.health <= 0.0 {
                        let respawn = RespawnEntry {
                            kind: RespawnKind::Mob {
                                m_type: mob.m_type.clone(),
                                level: mob.level,
                            },
                            remaining: config.spawning.mob_respawn_seconds,
                        };
                        chunk.mobs.remove(&mob_id);
                        chunk.cooldowns.push(respawn);
                        mob_killed = true;
                    }
                }
            }
        }
    }

    if attacked_mob {
        if mob_killed {
            if let Some(player) = world.get_player_mut(&player_id) {
                player.stats.kills = player.stats.kills.saturating_add(1);
            }
        }
        return;
    }

    if !config.balance.combat.pvp_enabled {
        return;
    }
    if world.is_in_safe_zone(player_x, player_y) {
        return;
    }

    let target_player = world
        .players
        .iter()
        .filter(|(id, other)| **id != player_id && other.spawned)
        .min_by(|(_, a), (_, b)| {
            let da = ((a.x - player_x).powi(2) + (a.y - player_y).powi(2)).sqrt();
            let db = ((b.x - player_x).powi(2) + (b.y - player_y).powi(2)).sqrt();
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(id, _)| *id);

    if let Some(target_id) = target_player {
        if let Some(target) = world.get_player_mut(&target_id) {
            let dx = target.x - player_x;
            let dy = target.y - player_y;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist <= ACTION_RANGE {
                target.health -= base_damage;
            }
        }
    }
}

pub fn try_npc_interaction(world: &World, player_id: PlayerId, outbox: &mut Vec<OutboundMessage>) {
    let Some(player) = world.get_player(&player_id) else {
        return;
    };
    let coord = ChunkCoord::new(player.chunk.0, player.chunk.1);
    let Some(chunk) = world.chunks.get(&coord) else {
        return;
    };

    let mut best_npc = None;
    let mut best_dist = f32::MAX;
    for npc in chunk.npcs.values() {
        let dx = npc.x - player.x;
        let dy = npc.y - player.y;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist < best_dist {
            best_dist = dist;
            best_npc = Some(npc);
        }
    }

    let Some(npc) = best_npc else {
        return;
    };
    if best_dist > ACTION_RANGE {
        return;
    }

    let data = NpcInteractionData {
        npc_id: npc.id.clone(),
        name: npc.name.clone(),
        text: "Need supplies?".to_string(),
        options: vec!["Browse".to_string(), "Goodbye".to_string()],
    };
    outbox.push((player_id, ServerMessage::NpcInteraction { data }));
}

fn stat_bonus(world: &World, player_id: &PlayerId, key: &str) -> f32 {
    world
        .get_player(player_id)
        .map(|player| {
            player
                .stat_bonuses
                .iter()
                .filter(|bonus| bonus.stat == key)
                .map(|bonus| bonus.value)
                .sum()
        })
        .unwrap_or(0.0)
}
