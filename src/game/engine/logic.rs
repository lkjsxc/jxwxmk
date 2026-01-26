use std::collections::HashMap;
use uuid::Uuid;
use actix::prelude::*;
use rand::Rng;
use crate::game::engine::{GameEngine, messages::ServerMessage};
use crate::game::entities::resource::{Resource, ResourceType};
use crate::game::entities::mob::{Mob, MobType};
use crate::game::systems::survival::SurvivalSystem;
use crate::game::systems::achievements::AchievementSystem;

impl GameEngine {
    pub(super) fn broadcast(&self) { 
        let msg = ServerMessage::WorldUpdate(self.world.clone()); 
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
        let unit_area = 10000.0;
        
        let resource_count = (area / unit_area * self.config.spawning.resource_density) as usize;
        let mob_count = (area / unit_area * self.config.spawning.mob_density) as usize;

        for _ in 0..resource_count {
            let x = rng.gen_range(0.0..self.world.width); let y = rng.gen_range(0.0..self.world.height);
            let r_type = match rng.gen_range(0..10) { 0..=4 => ResourceType::Tree, 5..=8 => ResourceType::Rock, _ => ResourceType::Food };
            let res = Resource::new(r_type, x, y); self.world.resources.insert(res.id, res);
        }
        for _ in 0..mob_count {
            let x = rng.gen_range(0.0..self.world.width); let y = rng.gen_range(0.0..self.world.height);
            let m_type = match rng.gen_range(0..10) { 0..=5 => MobType::Rabbit, 6..=8 => MobType::Wolf, _ => MobType::Bear };
            let mut mob = Mob::new(m_type, x, y); 
            let cx = self.world.width / 2.0; let cy = self.world.height / 2.0;
            let dist = ((x - cx).powi(2) + (y - cy).powi(2)).sqrt();
            let level = 1 + (dist * self.config.leveling.mob_level_factor) as u32;
            mob.level = level;
            mob.health *= 1.0 + (level as f64 * 0.2); 
            self.world.mobs.insert(mob.id, mob);
        }
    }
    
    pub(super) fn tick_world(&mut self) {
        let mut dead_p = Vec::new();
        for (id, p) in self.world.players.iter_mut() { 
            if !p.spawned { continue; }
            SurvivalSystem::tick(p, &self.config.mechanics); 
            if p.health <= 0.0 { dead_p.push(*id); } 
        }
        for id in dead_p { 
            if let Some(p) = self.world.players.get_mut(&id) { 
                p.stats.deaths += 1; 
                p.spawned = false; 
            }
        }

        let mut rng = rand::thread_rng();
        let player_ids: Vec<Uuid> = self.world.players.keys().cloned().collect();
        for mob in self.world.mobs.values_mut() {
            if mob.m_type == MobType::Rabbit {
                let dx = rng.gen_range(-1.0..1.0); let dy = rng.gen_range(-1.0..1.0);
                mob.x = (mob.x + dx).clamp(0.0, self.world.width); mob.y = (mob.y + dy).clamp(0.0, self.world.height);
            } else {
                let mut target_pos = None; let mut min_dist = 300.0;
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
        
        let mut dmg_to_apply = Vec::new();
        for mob in self.world.mobs.values() {
            if mob.m_type == MobType::Rabbit { continue; }
            for (pid, p) in self.world.players.iter() {
                if !p.spawned { continue; }
                if Math::dist(mob.x, mob.y, p.x, p.y) < 30.0 {
                    let base_dmg = match mob.m_type { MobType::Wolf => 0.5, MobType::Bear => 1.5, _ => 0.0 };
                    let level_mult = 1.0 + (mob.level as f64 * 0.1);
                    dmg_to_apply.push((*pid, base_dmg * level_mult));
                }
            }
        }
        for (pid, d) in dmg_to_apply { 
            if let Some(p) = self.world.players.get_mut(&pid) { 
                p.health -= d; p.stats.damage_taken += d; 
                if p.health <= 0.0 { p.spawned = false; p.stats.deaths += 1; }
            }
            self.check_achievements(pid);
        }
    }
}

pub(super) struct Math;
impl Math { pub(super) fn dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 { ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt() } }
