use crate::game::state::World;
use crate::game::config::AppConfig;
use crate::game::entities::item::ItemType;
use crate::game::entities::structure::{Structure, StructureType};
use crate::game::entities::resource::ResourceType;
use uuid::Uuid;

pub struct InteractionSystem;

impl InteractionSystem {
    pub fn handle_movement(world: &mut World, player_id: Uuid, dx: f64, dy: f64) {
        if let Some(player) = world.players.get_mut(&player_id) {
            if dx != 0.0 || dy != 0.0 { player.stats.steps_taken += 1; }
            let bonus_speed = *player.stat_bonuses.get("speed").unwrap_or(&0.0) as f64;
            let speed = 5.0 * (1.0 + bonus_speed);
            player.x = (player.x + dx * speed).clamp(0.0, world.width); 
            player.y = (player.y + dy * speed).clamp(0.0, world.height);
        }
    }

    pub fn handle_attack(world: &mut World, config: &AppConfig, player_id: Uuid, now: u64) {
        let (px, py, slot, mut tool_dmg, mut rock_mult, mut bonus_gather) = {
            if let Some(p) = world.players.get(&player_id) {
                if now - p.last_attack_at < config.mechanics.attack_cooldown { return; }
                let bd = *p.stat_bonuses.get("damage").unwrap_or(&0.0) as f64;
                let bg = *p.stat_bonuses.get("gather").unwrap_or(&0.0) as f64;
                let mut td = 2.0; let mut rm = 1.0;
                let mut level_bonus = 0.0;
                if let Some(item) = &p.inventory.slots[p.active_slot] {
                    if item.kind == ItemType::WoodPickaxe { td = 4.0; rm = 2.0; }
                    if item.kind == ItemType::StonePickaxe { td = 8.0; rm = 3.0; }
                    level_bonus = (item.level as f64 - 1.0) * 0.1; // 10% per level above 1
                }
                (p.x, p.y, p.active_slot, td * (1.0 + bd + level_bonus), rm, bg)
            } else { return; }
        };

        if let Some(p) = world.players.get_mut(&player_id) {
            p.last_attack_at = now;
            // Handle Item Use (Food/Building)
            let mut consumed = false;
            let mut built_structure: Option<Structure> = None;
            
            if let Some(item) = &mut p.inventory.slots[slot] {
                if matches!(item.kind, ItemType::Berry | ItemType::Meat | ItemType::CookedMeat) {
                     p.hunger = (p.hunger + config.mechanics.food_value).min(100.0);
                     item.amount -= 1; consumed = true;
                } else if let Some(st) = match item.kind {
                    ItemType::WoodWall => Some(StructureType::Wall), ItemType::Door => Some(StructureType::Door),
                    ItemType::Torch => Some(StructureType::Torch), ItemType::Workbench => Some(StructureType::Workbench), _ => None
                } {
                    built_structure = Some(Structure::new(st, p.x, p.y, player_id));
                    p.stats.structures_placed += 1;
                    item.amount -= 1; consumed = true;
                }
                if item.amount == 0 { p.inventory.slots[slot] = None; }
            }
            if let Some(s) = built_structure { world.structures.insert(s.id, s); return; } // Placed, done.
            if consumed { return; } // Ate, done.
        }

        // Raycast / Hit Detection
        let range = config.game.interact_range;
        
        // Helper to update tool XP
        let update_tool_xp = |p: &mut crate::game::entities::player::Player, slot: usize, xp_gain: f64| {
            if let Some(item) = &mut p.inventory.slots[slot] {
                if matches!(item.kind, ItemType::WoodPickaxe | ItemType::StonePickaxe) {
                    item.xp += xp_gain;
                    let threshold = 100.0 * (item.level as f64);
                    if item.xp >= threshold {
                        item.level += 1;
                        item.xp -= threshold;
                    }
                }
            }
        };

        // Resources
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
                    update_tool_xp(p, slot, config.leveling.tool_xp_per_use);
                }
                if r.amount <= 0 {
                    let drop = match r.r_type { ResourceType::Tree => (ItemType::Wood, 5), ResourceType::Rock => (ItemType::Stone, 3), ResourceType::Food => (ItemType::Berry, 2) };
                    if let Some(p) = world.players.get_mut(&player_id) { 
                        p.inventory.add(drop.0, drop.1); 
                        p.stats.resources_gathered += 1;
                    }
                    world.resources.remove(&rid);
                }
            }
            return;
        }

        // Mobs
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
                    update_tool_xp(p, slot, config.leveling.tool_xp_per_use * 2.0); // More XP for combat?
                }
                if m.health <= 0.0 {
                    if let Some(p) = world.players.get_mut(&player_id) {
                         p.inventory.add(ItemType::Meat, 2); 
                         p.stats.mobs_killed += 1;
                    }
                    world.mobs.remove(&mid);
                }
            }
            return;
        }
        
        // Players (PVP)
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
    }
}