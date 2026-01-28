use world::{World, ChunkCoord, Entity, EntityKind, Vec2, InventorySlot};
use std::time::{Duration, Instant};
use uuid::Uuid;

pub fn tick(world: &mut World, dt: Duration) {
    let dt_secs = dt.as_secs_f32();
    let base_speed = world.config.balance.player.base_speed;

    // 1. Movement
    for player in world.players.values_mut() {
        let dx = player.input_dx.clamp(-1.0, 1.0);
        let dy = player.input_dy.clamp(-1.0, 1.0);
        
        let mag = (dx*dx + dy*dy).sqrt();
        let (nx, ny) = if mag > 1.0 {
            (dx / mag, dy / mag)
        } else {
            (dx, dy)
        };

        let move_x = nx * base_speed * dt_secs;
        let move_y = ny * base_speed * dt_secs;
        player.pos.x += move_x;
        player.pos.y += move_y;

        if move_x.abs() > 0.0 || move_y.abs() > 0.0 {
            let dist = (move_x*move_x + move_y*move_y).sqrt();
            *player.stats.entry("steps".to_string()).or_insert(0.0) += dist;
        }
    }

    // 2. Resolve Actions
    let action_players: Vec<Uuid> = world.players.values()
        .filter(|p| p.input_attack || p.input_interact)
        .map(|p| p.id)
        .collect();

    for pid in action_players {
        resolve_action(world, pid);
    }
}

fn resolve_action(world: &mut World, player_id: Uuid) {
    let player = if let Some(p) = world.players.get(&player_id) { p.clone() } else { return };
    let aim = if let Some(a) = player.input_aim { a } else { return };

    let dist_sq = (player.pos.x - aim.x).powi(2) + (player.pos.y - aim.y).powi(2);
    let range = world.config.balance.player.interaction_range_wu;
    if dist_sq > range * range { return; }

    let active_item = player.inventory.get(player.active_slot).and_then(|s| s.as_ref());

    if player.input_interact {
        if try_interact_npc(world, player_id, aim) { return; }

        if let Some(item) = active_item {
            if is_consumable(&item.item_id) {
                consume_item(world, player_id);
                return;
            }
            if is_structure(&item.item_id) {
                place_structure(world, player_id, aim);
                return;
            }
        }
    }

    if player.input_attack {
        if try_gather_resource(world, player_id, aim) { return; }
        try_combat(world, player_id, aim);
    }
}

fn is_consumable(id: &str) -> bool {
    id.contains("berry") || id.contains("meat") || id.contains("potion") || id.contains("stew")
}

fn is_structure(id: &str) -> bool {
    id.contains("wall") || id.contains("door") || id.contains("workbench") || id.contains("campfire")
}

fn try_interact_npc(world: &mut World, _player_id: Uuid, aim: Vec2) -> bool {
    let chunk_size = world.config.world.chunk_size_wu as f32;
    let cx = (aim.x / chunk_size).floor() as i32;
    let cy = (aim.y / chunk_size).floor() as i32;
    let coord = ChunkCoord { x: cx, y: cy };

    if let Some(chunk) = world.get_chunk(coord) {
        for npc in chunk.npcs.values() {
            let nx = npc.pos.x + (coord.x as f32 * chunk_size);
            let ny = npc.pos.y + (coord.y as f32 * chunk_size);
            let d = ((nx - aim.x).powi(2) + (ny - aim.y).powi(2)).sqrt();
            if d < 1.5 {
                return true;
            }
        }
    }
    false
}

fn try_gather_resource(world: &mut World, player_id: Uuid, aim: Vec2) -> bool {
    let chunk_size = world.config.world.chunk_size_wu as f32;
    let cx = (aim.x / chunk_size).floor() as i32;
    let cy = (aim.y / chunk_size).floor() as i32;
    let coord = ChunkCoord { x: cx, y: cy };

    let mut target_id = None;
    if let Some(chunk) = world.get_chunk(coord) {
        let mut min_dist = 2.0;
        for res in chunk.resources.values() {
            let rx = res.pos.x + (coord.x as f32 * chunk_size);
            let ry = res.pos.y + (coord.y as f32 * chunk_size);
            let d = ((rx - aim.x).powi(2) + (ry - aim.y).powi(2)).sqrt();
            if d < min_dist {
                min_dist = d;
                target_id = Some(res.id);
            }
        }
    }

    if let Some(id) = target_id {
        let mut drop = None;
        if let Some(chunk) = world.get_chunk_mut(coord) {
            if let Some(res) = chunk.resources.get_mut(&id) {
                res.hp -= 10.0;
                if res.hp <= 0.0 {
                    let subtype = res.subtype.clone();
                    let ent = res.clone();
                    chunk.resources.remove(&id);
                    chunk.respawn_queue.push((Instant::now() + Duration::from_secs(60), ent));
                    drop = Some((subtype, 5));
                    chunk.dirty = true;
                }
            }
        }
        if let Some((subtype, count)) = drop {
            crate::quests::update_quests(world, player_id, "gather", &subtype, count);
            let p = world.players.get_mut(&player_id).unwrap();
            let item_id = match subtype.as_str() {
                "tree" => "wood",
                "rock" => "stone",
                _ => "berry",
            };
            add_to_inventory(p, item_id, count);
        }
        return true;
    }
    false
}

fn try_combat(world: &mut World, player_id: Uuid, aim: Vec2) -> bool {
    let chunk_size = world.config.world.chunk_size_wu as f32;
    let cx = (aim.x / chunk_size).floor() as i32;
    let cy = (aim.y / chunk_size).floor() as i32;
    let coord = ChunkCoord { x: cx, y: cy };

    let mut target_id = None;
    if let Some(chunk) = world.get_chunk(coord) {
        let mut min_dist = 1.5;
        for mob in chunk.mobs.values() {
            let mx = mob.pos.x + (coord.x as f32 * chunk_size);
            let my = mob.pos.y + (coord.y as f32 * chunk_size);
            let d = ((mx - aim.x).powi(2) + (my - aim.y).powi(2)).sqrt();
            if d < min_dist {
                min_dist = d;
                target_id = Some(mob.id);
            }
        }
    }

    if let Some(id) = target_id {
        let mut killed_subtype = None;
        if let Some(chunk) = world.get_chunk_mut(coord) {
            if let Some(mob) = chunk.mobs.get_mut(&id) {
                mob.hp -= 20.0;
                if mob.hp <= 0.0 {
                    let subtype = mob.subtype.clone();
                    let ent = mob.clone();
                    chunk.mobs.remove(&id);
                    chunk.respawn_queue.push((Instant::now() + Duration::from_secs(120), ent));
                    killed_subtype = Some(subtype);
                    chunk.dirty = true;
                }
            }
        }
        if let Some(subtype) = killed_subtype {
            crate::quests::update_quests(world, player_id, "kill", &subtype, 1);
        }
        return true;
    }
    false
}

fn consume_item(world: &mut World, player_id: Uuid) {
    let player = world.players.get_mut(&player_id).unwrap();
    let slot_idx = player.active_slot;
    if let Some(slot) = player.inventory[slot_idx].as_mut() {
        if slot.item_id.contains("berry") {
            player.hunger = (player.hunger + 10.0).min(100.0);
        }
        slot.count -= 1;
        if slot.count == 0 {
            player.inventory[slot_idx] = None;
        }
    }
}

fn place_structure(world: &mut World, player_id: Uuid, aim: Vec2) {
    let grid_size = 2.5;
    let sx = (aim.x / grid_size).round() * grid_size;
    let sy = (aim.y / grid_size).round() * grid_size;
    
    let player = world.players.get_mut(&player_id).unwrap();
    let item_id = player.inventory[player.active_slot].as_ref().unwrap().item_id.clone();
    
    if let Some(slot) = player.inventory[player.active_slot].as_mut() {
        slot.count -= 1;
        if slot.count == 0 { player.inventory[player.active_slot] = None; }
    }

    let chunk_size = world.config.world.chunk_size_wu as f32;
    let cx = (sx / chunk_size).floor() as i32;
    let cy = (sy / chunk_size).floor() as i32;
    let coord = ChunkCoord { x: cx, y: cy };

    if let Some(chunk) = world.get_chunk_mut(coord) {
        let id = Uuid::new_v4();
        chunk.structures.insert(id, Entity {
            id,
            kind: EntityKind::Structure,
            subtype: item_id,
            pos: Vec2 { x: sx - (cx as f32 * chunk_size), y: sy - (cy as f32 * chunk_size) },
            hp: 100.0,
            max_hp: 100.0,
            name: None,
            owner_id: Some(player_id),
            target_pos: None,
        });
        chunk.dirty = true;
    }
}

fn add_to_inventory(player: &mut world::PlayerState, item_id: &str, count: u32) {
    for slot in player.inventory.iter_mut().flatten() {
        if slot.item_id == item_id {
            slot.count += count;
            return;
        }
    }
    for slot in player.inventory.iter_mut() {
        if slot.is_none() {
            *slot = Some(InventorySlot { item_id: item_id.to_string(), count });
            return;
        }
    }
}
