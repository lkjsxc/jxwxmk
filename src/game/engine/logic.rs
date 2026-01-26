use std::collections::HashMap;
use uuid::Uuid;
use actix::prelude::*;
use rand::Rng;
use crate::game::engine::{GameEngine, messages::ServerMessage};
use crate::game::entities::resource::{Resource, ResourceType};
use crate::game::entities::mob::{Mob, MobType};
use crate::game::entities::npc::{Npc, NpcType};
use crate::game::entities::barrier::BarrierCore;
use crate::game::entities::item::{Item, ItemType};
use crate::game::systems::survival::SurvivalSystem;
use crate::game::systems::achievements::AchievementSystem;

impl GameEngine {
    pub(super) fn broadcast(&self) { 
        let mut world_copy = self.world.clone();
        world_copy.players.retain(|_, p| p.spawned);
        let msg = ServerMessage::WorldUpdate(world_copy); 
        for addr in self.sessions.values() { 
            addr.do_send(msg.clone()); 
        } 
    }
    
    pub(super) fn check_achievements(&mut self, player_id: Uuid) {
        if let Some(player) = self.world.players.get_mut(&player_id) {
            let unlocked = AchievementSystem::check(player);
            if !unlocked.is_empty() {
                if let Some(addr) = self.sessions.get(&player_id) {
                    for ach in unlocked { addr.do_send(ServerMessage::AchievementUnlocked(ach)); }
                }
            }
        }
    }

    pub(super) fn spawn_initial_entities(&mut self) {
        let mut rng = rand::thread_rng();
        let area = self.world.width * self.world.height;
        let unit_area = self.config.spawning.unit_area;
        
        let resource_count = (area / unit_area * self.config.spawning.resource_density) as usize;
        let mob_count = (area / unit_area * self.config.spawning.mob_density) as usize;

        for _ in 0..resource_count {
            let x = rng.gen_range(0.0..self.world.width); let y = rng.gen_range(0.0..self.world.height);
            let r_type = match rng.gen_range(0..10) { 0..=4 => ResourceType::Tree, 5..=8 => ResourceType::Rock, _ => ResourceType::Food };
            let res = Resource::new_with_config(r_type, x, y, &self.config.balance.resources); self.world.resources.insert(res.id, res);
        }
        for _ in 0..mob_count {
            let x = rng.gen_range(0.0..self.world.width); let y = rng.gen_range(0.0..self.world.height);
            let m_type = match rng.gen_range(0..10) { 0..=5 => MobType::Rabbit, 6..=8 => MobType::Wolf, _ => MobType::Bear };
            let mut mob = Mob::new_with_config(m_type, x, y, &self.config.balance.mobs); 
            let cx = self.world.width / 2.0; let cy = self.world.height / 2.0;
            let dist = ((x - cx).powi(2) + (y - cy).powi(2)).sqrt();
            let level = 1 + (dist * self.config.leveling.mob_level_factor) as u32;
            mob.level = level;
            mob.health *= 1.0 + (level as f64 * self.config.balance.mobs.level_hp_mult); 
            self.world.mobs.insert(mob.id, mob);
        }

        // Spawn Barrier Cores
        let cx = self.world.width / 2.0;
        let cy = self.world.height / 2.0;

        // 1. Always spawn at center
        let center_core = BarrierCore::new(cx, cy, 1, self.config.barriers.base_range);
        self.world.barrier_cores.insert(center_core.id, center_core);

        // 2. Probabilistic spawning for additional barriers
        let mut additional_barriers = 0;
        for _ in 0..self.config.barriers.max_additional_barriers * 10 { // Sample points
            if additional_barriers >= self.config.barriers.max_additional_barriers { break; }
            
            let x = rng.gen_range(0.0..self.world.width);
            let y = rng.gen_range(0.0..self.world.height);
            
            let dist = ((x - cx).powi(2) + (y - cy).powi(2)).sqrt();
            let max_dist = ((self.world.width/2.0).powi(2) + (self.world.height/2.0).powi(2)).sqrt();
            
            // Higher probability closer to center
            let prob = (1.0 - (dist / max_dist)).powi(2) * self.config.barriers.placement_chance_center;
            
            if rng.gen_bool(prob.clamp(0.0, 1.0)) {
                let level = rng.gen_range(1..=3);
                let core = BarrierCore::new(x, y, level, self.config.barriers.base_range);
                self.world.barrier_cores.insert(core.id, core);
                additional_barriers += 1;
            }
        }

        // Spawn Village NPCs within barriers
        for core in self.world.barrier_cores.values() {
            if core.x == cx && core.y == cy {
                // Center village
                let elder = Npc::new(NpcType::Elder, "Elder", core.x + 20.0, core.y + 20.0);
                self.world.npcs.insert(elder.id, elder);
                let merchant = Npc::new(NpcType::Merchant, "Merchant", core.x - 20.0, core.y - 20.0);
                self.world.npcs.insert(merchant.id, merchant);
            } else {
                // Secondary villages
                if rng.gen_bool(0.5) {
                    let merchant = Npc::new(NpcType::Merchant, "Villager", core.x + rng.gen_range(-30.0..30.0), core.y + rng.gen_range(-30.0..30.0));
                    self.world.npcs.insert(merchant.id, merchant);
                }
            }
        }
    }
    
    pub(super) fn tick_world(&mut self) {
        // 0. Barrier check (Eliminate hostile mobs in range)
        let mut mobs_to_kill = Vec::new();
        for (mid, mob) in self.world.mobs.iter() {
            if mob.m_type == MobType::Rabbit { continue; }
            for core in self.world.barrier_cores.values() {
                let dist = Math::dist(mob.x, mob.y, core.x, core.y);
                if dist < core.range(self.config.barriers.level_multiplier) {
                    mobs_to_kill.push(*mid);
                    break;
                }
            }
        }
        for mid in mobs_to_kill { self.world.mobs.remove(&mid); }

        // 1. Survival tick
        for p in self.world.players.values_mut() { 
            if p.spawned { SurvivalSystem::tick(p, &self.config); }
        }

        // 2. Mob AI
        let mut rng = rand::thread_rng();
        let player_ids: Vec<Uuid> = self.world.players.keys().cloned().collect();
        for mob in self.world.mobs.values_mut() {
            if mob.m_type == MobType::Rabbit {
                let dx = rng.gen_range(-1.0..1.0); let dy = rng.gen_range(-1.0..1.0);
                mob.x = (mob.x + dx).clamp(0.0, self.world.width); mob.y = (mob.y + dy).clamp(0.0, self.world.height);
            } else {
                let mut target_pos = None; let mut min_dist = self.config.balance.mobs.aggression_range;
                for pid in &player_ids {
                    if let Some(p) = self.world.players.get(pid) {
                        if !p.spawned { continue; }
                        let d = Math::dist(mob.x, mob.y, p.x, p.y);
                        if d < min_dist { min_dist = d; target_pos = Some((p.x, p.y)); }
                    }
                }
                if let Some((tx, ty)) = target_pos {
                    let dx = tx - mob.x; let dy = ty - mob.y;
                    if min_dist > 20.0 { mob.x += dx / min_dist * 2.0; mob.y += dy / min_dist * 2.0; }
                }
            }
        }
        
        // 3. Mob Damage
        let mut dmg_to_apply = Vec::new();
        for mob in self.world.mobs.values() {
            if mob.m_type == MobType::Rabbit { continue; }
            for (pid, p) in self.world.players.iter() {
                if !p.spawned { continue; }
                if Math::dist(mob.x, mob.y, p.x, p.y) < self.config.balance.mobs.attack_range {
                    let base_dmg = match mob.m_type { MobType::Wolf => self.config.balance.mobs.wolf_dmg, MobType::Bear => self.config.balance.mobs.bear_dmg, _ => 0.0 };
                    let level_mult = 1.0 + (mob.level as f64 * self.config.balance.mobs.level_dmg_mult);
                    dmg_to_apply.push((*pid, base_dmg * level_mult));
                }
            }
        }
        for (pid, d) in dmg_to_apply { 
            if let Some(p) = self.world.players.get_mut(&pid) { 
                p.health -= d; p.stats.damage_taken += d; 
            }
            self.check_achievements(pid);
        }

        // 4. Death Check & Cleanup (Catch all deaths)
        let mut dead_ids = Vec::new();
        for (id, p) in self.world.players.iter_mut() {
            if p.spawned && p.health <= 0.0 {
                dead_ids.push(*id);
            }
        }
        for id in dead_ids {
            if let Some(p) = self.world.players.get_mut(&id) {
                p.spawned = false;
                p.health = 0.0;
                p.stats.deaths += 1;
                p.inventory = crate::game::entities::player::Inventory::default();
            }
        }
    }
}

pub(super) struct Math;
impl Math { pub(super) fn dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 { ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt() } }
