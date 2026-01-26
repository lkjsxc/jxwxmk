use crate::game::state::World;
use crate::game::config::AppConfig;
use crate::game::entities::item::ItemType;
use crate::game::entities::structure::{Structure, StructureType};
use crate::game::entities::resource::ResourceType;
use uuid::Uuid;

pub struct InteractionSystem;

pub enum InteractionEvent {
    LevelUp { tool: String, level: u32 },
    Npc { npc_id: Uuid },
    Gather { item: ItemType, amount: u32 },
    Kill { mob_type: String },
}

impl InteractionSystem {
    pub fn handle_interact(world: &mut World, config: &AppConfig, player_id: Uuid) -> Option<InteractionEvent> {
        let (px, py) = {
            let p = world.players.get(&player_id)?;
            (p.x, p.y)
        };

        let range = config.game.interact_range;
        let mut target_npc = None;
        let mut min_d = range;

        for (id, n) in world.npcs.iter() {
            let d = ((px - n.x).powi(2) + (py - n.y).powi(2)).sqrt();
            if d < min_d {
                min_d = d;
                target_npc = Some(*id);
            }
        }

        target_npc.map(|npc_id| InteractionEvent::Npc { npc_id })
    }

    pub fn handle_movement(world: &mut World, config: &AppConfig, player_id: Uuid, dx: f64, dy: f64) {
        if let Some(player) = world.players.get_mut(&player_id) {
            if dx != 0.0 || dy != 0.0 { player.stats.steps_taken += 1; }
            let bonus_speed = *player.stat_bonuses.get("speed").unwrap_or(&0.0) as f64;
            let speed = config.balance.player.base_speed * (1.0 + bonus_speed);
            player.x = (player.x + dx * speed).clamp(0.0, world.width); 
            player.y = (player.y + dy * speed).clamp(0.0, world.height);
        }
    }

    pub fn handle_attack(world: &mut World, config: &AppConfig, player_id: Uuid, now: u64) -> Vec<InteractionEvent> {
        let mut events = Vec::new();
        
        let (px, py, slot, mut tool_dmg, mut rock_mult, mut bonus_gather) = {
            if let Some(p) = world.players.get(&player_id) {
                if now - p.last_attack_at < (config.mechanics.attack_cooldown * 1000.0) as u64 { return events; }
                let bd = *p.stat_bonuses.get("damage").unwrap_or(&0.0) as f64;
                let bg = *p.stat_bonuses.get("gather").unwrap_or(&0.0) as f64;
                let mut td = config.balance.tools.base_dmg; let mut rm = 1.0;
                let mut level_bonus = 0.0;
                if let Some(item) = &p.inventory.slots[p.active_slot] {
                    if item.kind == ItemType::WoodPickaxe { td = config.balance.tools.wood_pickaxe_dmg; rm = config.balance.tools.rock_mult; }
                    if item.kind == ItemType::StonePickaxe { td = config.balance.tools.stone_pickaxe_dmg; rm = config.balance.tools.rock_mult; }
                    level_bonus = (item.level as f64 - 1.0) * config.balance.tools.tool_level_dmg_bonus;
                }
                (p.x, p.y, p.active_slot, td * (1.0 + bd + level_bonus), rm, bg)
            } else { return events; }
        };

        if let Some(p) = world.players.get_mut(&player_id) {
            p.last_attack_at = now;
            let mut consumed = false;
            let mut built_structure: Option<Structure> = None;
            
            if let Some(item) = &mut p.inventory.slots[slot] {
                if matches!(item.kind, ItemType::Berry | ItemType::Meat | ItemType::CookedMeat) {
                     p.hunger = (p.hunger + config.mechanics.food_value).min(config.balance.player.max_hunger);
                     item.amount -= 1; consumed = true;
                } else if let Some(st) = match item.kind {
                    ItemType::WoodWall => Some(StructureType::Wall), ItemType::Door => Some(StructureType::Door),
                    ItemType::Torch => Some(StructureType::Torch), ItemType::Workbench => Some(StructureType::Workbench), _ => None
                } {
                    built_structure = Some(Structure::new_with_config(st, p.x, p.y, player_id, &config.balance.structures));
                    p.stats.structures_placed += 1;
                    item.amount -= 1; consumed = true;
                }
                if item.amount == 0 { p.inventory.slots[slot] = None; }
            }
            if let Some(s) = built_structure { world.structures.insert(s.id, s); return events; } 
            if consumed { return events; } 
        }

        let range = config.game.interact_range;
        
        let mut target_res = None;
        let mut min_d = range;
        for (id, r) in world.resources.iter() {
            let d = ((px - r.x).powi(2) + (py - r.y).powi(2)).sqrt();
            if d < min_d { min_d = d; target_res = Some(*id); }
        }
        if let Some(rid) = target_res {
            if let Some(r) = world.resources.get_mut(&rid) {
                let gm = if r.r_type == ResourceType::Rock { rock_mult } else { 1.0 };
                r.amount -= (tool_dmg * gm * (1.0 + bonus_gather)) as i32;
                
                if let Some(p) = world.players.get_mut(&player_id) {
                    if let Some(item) = &mut p.inventory.slots[slot] {
                        if matches!(item.kind, ItemType::WoodPickaxe | ItemType::StonePickaxe) {
                            item.xp += config.leveling.tool_xp_per_use;
                            let threshold = 100.0 * (item.level as f64);
                            if item.xp >= threshold {
                                item.level += 1;
                                item.xp -= threshold;
                                events.push(InteractionEvent::LevelUp { tool: format!("{:?}", item.kind), level: item.level });
                            }
                        }
                    }
                }
                
                if r.amount <= 0 {
                    let drop = match r.r_type { ResourceType::Tree => (ItemType::Wood, 5), ResourceType::Rock => (ItemType::Stone, 3), ResourceType::Food => (ItemType::Berry, 2) };
                    events.push(InteractionEvent::Gather { item: drop.0.clone(), amount: drop.1 });
                    if let Some(p) = world.players.get_mut(&player_id) { 
                        p.inventory.add(drop.0, drop.1); 
                        p.stats.resources_gathered += 1;
                    }
                    world.resources.remove(&rid);
                }
            }
            return events;
        }

        let mut target_mob = None;
        min_d = range;
        for (id, m) in world.mobs.iter() {
            let d = ((px - m.x).powi(2) + (py - m.y).powi(2)).sqrt();
            if d < min_d { min_d = d; target_mob = Some(*id); }
        }
        if let Some(mid) = target_mob {
            if let Some(m) = world.mobs.get_mut(&mid) {
                m.health -= tool_dmg;
                
                if let Some(p) = world.players.get_mut(&player_id) {
                    if let Some(item) = &mut p.inventory.slots[slot] {
                        if matches!(item.kind, ItemType::WoodPickaxe | ItemType::StonePickaxe) {
                            item.xp += config.leveling.tool_xp_per_use * 2.0;
                            let threshold = 100.0 * (item.level as f64);
                            if item.xp >= threshold {
                                item.level += 1;
                                item.xp -= threshold;
                                events.push(InteractionEvent::LevelUp { tool: format!("{:?}", item.kind), level: item.level });
                            }
                        }
                    }
                }

                                if m.health <= 0.0 {
                                    events.push(InteractionEvent::Kill { mob_type: format!("{:?}", m.m_type) });
                                    if let Some(p) = world.players.get_mut(&player_id) { 
                                         p.inventory.add(ItemType::Meat, 2); 
                                         p.stats.mobs_killed += 1;
                                    }
                                    world.mobs.remove(&mid);
                                }            }
            return events;
        }
        
        let mut target_p = None;
        min_d = range;
        for (id, p) in world.players.iter() {
            if *id == player_id { continue; }
            let d = ((px - p.x).powi(2) + (py - p.y).powi(2)).sqrt();
            if d < min_d { min_d = d; target_p = Some(*id); }
        }
        if let Some(pid) = target_p {
            if let Some(p) = world.players.get_mut(&pid) {
                p.health -= tool_dmg;
                p.stats.damage_taken += tool_dmg;
            }
        }
        
        events
    }
}
