use crate::game::world_state::{World, MobType, Player, Effect};
use crate::config::AppConfig;
use rand::Rng;
use uuid::Uuid;

pub fn tick_mob_ai(world: &mut World) {
    let config = AppConfig::get();
    let mut rng = rand::thread_rng();

    // 1. Move Mobs
    for mob in world.mobs.values_mut() {
        match mob.m_type {
            MobType::Rabbit => {
                // Random wander
                let dx: f64 = rng.gen_range(-1.0..1.0);
                let dy: f64 = rng.gen_range(-1.0..1.0);
                mob.x += dx * 2.0;
                mob.y += dy * 2.0;
            },
            MobType::Wolf | MobType::Bear => {
                // Find nearest player
                let mut target: Option<(&Player, f64)> = None;
                for player in world.players.values() {
                    if !player.spawned { continue; }
                    let d = ((player.x - mob.x).powi(2) + (player.y - mob.y).powi(2)).sqrt();
                    if d < config.balance.mobs.aggression_range {
                        if let Some((_, closest_d)) = target {
                            if d < closest_d {
                                target = Some((player, d));
                            }
                        } else {
                            target = Some((player, d));
                        }
                    }
                }

                if let Some((player, dist)) = target {
                    if dist > config.balance.mobs.attack_range {
                        let dx = (player.x - mob.x) / dist;
                        let dy = (player.y - mob.y) / dist;
                        mob.x += dx * 2.5; // Slightly faster than rabbit
                        mob.y += dy * 2.5;
                    }
                } else {
                    // Wander if no target
                    let dx: f64 = rng.gen_range(-0.5..0.5);
                    let dy: f64 = rng.gen_range(-0.5..0.5);
                    mob.x += dx;
                    mob.y += dy;
                }
            }
        }

        // Clamp bounds
        mob.x = mob.x.clamp(0.0, world.width);
        mob.y = mob.y.clamp(0.0, world.height);
    }

    // 2. Mob Damage (Direct player modification requires splitting borrows, skipping for now)
    // We will do damage in a separate pass in the engine to avoid borrow issues.
}

pub fn calculate_mob_damage_to_player(world: &mut World) {
    let config = AppConfig::get();
    
    // We need to iterate mobs and find players in range.
    // To avoid borrow checker hell (mutable world vs mutable players), we collect damage events first.
    let mut damage_events = Vec::new();

    for mob in world.mobs.values() {
        let is_hostile = matches!(mob.m_type, MobType::Wolf | MobType::Bear);
        if !is_hostile { continue; }

        let dmg = match mob.m_type {
            MobType::Wolf => config.balance.mobs.wolf_dmg,
            MobType::Bear => config.balance.mobs.bear_dmg,
            _ => 0.0,
        };
        // Level scaling: base * (1 + (level-1)*factor)
        let level_mult = 1.0 + ((mob.level as f64 - 1.0) * config.balance.mobs.level_dmg_mult);
        let final_dmg = dmg * level_mult;

        for player in world.players.values() {
            if !player.spawned { continue; }
            let d = ((player.x - mob.x).powi(2) + (player.y - mob.y).powi(2)).sqrt();
            if d <= config.balance.mobs.attack_range {
                damage_events.push((player.id, final_dmg));
            }
        }
    }

    // Apply damage
    for (pid, dmg) in damage_events {
        if let Some(p) = world.players.get_mut(&pid) {
            p.health = (p.health - dmg).max(0.0);
            p.stats.damage_taken += dmg;

            let eid = Uuid::new_v4();
            world.effects.insert(eid, Effect {
                id: eid,
                x: p.x,
                y: p.y - 20.0,
                text: format!("-{}", dmg as u32),
                color: "#f00".to_string(),
                ttl: 20,
            });

            if p.health <= 0.0 {
                p.spawned = false;
                p.stats.deaths += 1;
            }
        }
    }
}
