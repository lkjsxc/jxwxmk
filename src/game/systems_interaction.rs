use crate::game::world_state::{Player, World, Resource, Mob, Structure, ItemType, StructureType, Npc, Effect};
use crate::config::AppConfig;
use crate::game::systems_crafting::add_item;
use crate::game::quests::QuestEvent;
use uuid::Uuid;

pub fn handle_attack(player: &mut Player, world: &mut World) -> Vec<QuestEvent> {
    let config = AppConfig::get();
    let mut events = Vec::new();
    
    // Range check
    let range = config.game.interact_range;
    
    // 1. Consume Food (TODO)
    
    // 2. Place Structure
    if let Some(Some(item)) = player.inventory.slots.get_mut(player.active_slot) {
        let s_type = match item.kind {
            ItemType::WoodWall => Some(StructureType::Wall),
            ItemType::Door => Some(StructureType::Door),
            ItemType::Torch => Some(StructureType::Torch),
            ItemType::Workbench => Some(StructureType::Workbench),
            _ => None,
        };
        
        if let Some(st) = s_type {
            // Place it
            let id = Uuid::new_v4();
            world.structures.insert(id, Structure {
                id,
                s_type: st,
                x: player.x,
                y: player.y,
                health: config.balance.structures.wall_health, // Simplified
                owner_id: player.id,
            });
            player.stats.structures += 1;
            
            // Decrement item
            if item.amount > 1 {
                item.amount -= 1;
            } else {
                player.inventory.slots[player.active_slot] = None;
            }
            return events; // Action consumed
        }
    }

    // 3. Check resources
    let mut gathered_id = None;
    let mut dropped_items = Vec::new();

    for resource in world.resources.values_mut() {
        if dist(player.x, player.y, resource.x, resource.y) <= range {
            let dmg = calculate_resource_damage(player, resource);
            resource.amount -= dmg;
            
            let eid = Uuid::new_v4();
            world.effects.insert(eid, Effect {
                id: eid,
                x: resource.x,
                y: resource.y - 20.0,
                text: format!("-{}", dmg as u32),
                color: "#ff0".to_string(),
                ttl: 20,
            });

            if resource.amount <= 0.0 {
                gathered_id = Some(resource.id);
                match resource.r_type {
                    crate::game::world_state::ResourceType::Tree => {
                        dropped_items.push((ItemType::Wood, 5));
                        events.push(QuestEvent::Gather(ItemType::Wood, 5));
                    },
                    crate::game::world_state::ResourceType::Rock => {
                        dropped_items.push((ItemType::Stone, 3));
                        events.push(QuestEvent::Gather(ItemType::Stone, 3));
                    },
                    crate::game::world_state::ResourceType::Food => {
                        dropped_items.push((ItemType::Berry, 2));
                        events.push(QuestEvent::Gather(ItemType::Berry, 2));
                    },
                }
            }
            player.stats.gathers += 1;
            break;
        }
    }
    if let Some(id) = gathered_id {
        world.resources.remove(&id);
        for (kind, amount) in dropped_items {
            add_item(&mut player.inventory, kind, amount);
        }
        return events;
    }

    // 4. Check mobs
    let mut killed_mob_id = None;
    let mut mob_drops = Vec::new();
    for mob in world.mobs.values_mut() {
        if dist(player.x, player.y, mob.x, mob.y) <= range {
            let dmg = calculate_mob_damage(player, mob);
            mob.health -= dmg;

            let eid = Uuid::new_v4();
            world.effects.insert(eid, Effect {
                id: eid,
                x: mob.x,
                y: mob.y - 20.0,
                text: format!("-{}", dmg as u32),
                color: "#f00".to_string(),
                ttl: 20,
            });

            if mob.health <= 0.0 {
                killed_mob_id = Some(mob.id);
                mob_drops.push((ItemType::Meat, 2));
                events.push(QuestEvent::Kill(mob.m_type.clone()));
            }
            break;
        }
    }
    if let Some(id) = killed_mob_id {
        world.mobs.remove(&id);
        player.stats.kills += 1;
        for (kind, amount) in mob_drops {
            add_item(&mut player.inventory, kind, amount);
        }
    }
    
    events
}

pub fn handle_interact(player: &Player, world: &World) -> Option<Uuid> {
    let config = AppConfig::get();
    let range = config.game.interact_range;

    // Find closest NPC
    let mut closest: Option<(Uuid, f64)> = None;

    for npc in world.npcs.values() {
        let d = dist(player.x, player.y, npc.x, npc.y);
        if d <= range {
            if let Some((_, cd)) = closest {
                if d < cd {
                    closest = Some((npc.id, d));
                }
            } else {
                closest = Some((npc.id, d));
            }
        }
    }

    closest.map(|(id, _)| id)
}

fn dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}

fn calculate_resource_damage(player: &Player, resource: &Resource) -> f64 {
    let config = AppConfig::get();
    let mut dmg = config.balance.tools.base_dmg;
    
    if let Some(Some(item)) = player.inventory.slots.get(player.active_slot) {
        match item.kind {
            ItemType::WoodPickaxe => dmg = config.balance.tools.wood_pickaxe_dmg,
            ItemType::StonePickaxe => dmg = config.balance.tools.stone_pickaxe_dmg,
            _ => (),
        }
    }
    
    match resource.r_type {
        crate::game::world_state::ResourceType::Rock => dmg *= config.balance.tools.rock_mult,
        _ => (),
    }
    
    dmg
}

fn calculate_mob_damage(player: &Player, _mob: &crate::game::world_state::Mob) -> f64 {
    let config = AppConfig::get();
    let mut dmg = config.balance.tools.base_dmg;
    
    if let Some(Some(item)) = player.inventory.slots.get(player.active_slot) {
        match item.kind {
            ItemType::WoodPickaxe => dmg = config.balance.tools.wood_pickaxe_dmg,
            ItemType::StonePickaxe => dmg = config.balance.tools.stone_pickaxe_dmg,
            _ => (),
        }
    }
    
    dmg
}