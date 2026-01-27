use crate::game::world_state::{World, Resource, ResourceType, Mob, MobType, BarrierCore, Npc, NpcType};
use crate::config::AppConfig;
use uuid::Uuid;
use rand::Rng;

pub fn spawn_initial_entities(world: &mut World) {
    let config = AppConfig::get();
    let area = world.width * world.height;
    let unit_area = config.spawning.unit_area;
    
    let resource_count = (config.spawning.resource_density * (area / unit_area)) as usize;
    let mob_count = (config.spawning.mob_density * (area / unit_area)) as usize;
    
    let mut rng = rand::thread_rng();

    // Spawn Center Barrier
    let center_id = Uuid::new_v4();
    world.barrier_cores.insert(center_id, BarrierCore {
        id: center_id,
        x: world.width / 2.0,
        y: world.height / 2.0,
        level: 1,
        base_range: config.barriers.base_range,
    });

    // Spawn Center NPCs
    world.npcs.insert(Uuid::new_v4(), Npc {
        id: Uuid::new_v4(),
        n_type: NpcType::Elder,
        name: "Elder".to_string(),
        x: (world.width / 2.0) - 20.0,
        y: (world.height / 2.0) + 20.0,
        health: 100.0,
        dialogue_index: 0,
        trade_inventory: None,
    });
    world.npcs.insert(Uuid::new_v4(), Npc {
        id: Uuid::new_v4(),
        n_type: NpcType::Merchant,
        name: "Trader".to_string(),
        x: (world.width / 2.0) + 20.0,
        y: (world.height / 2.0) - 20.0,
        health: 100.0,
        dialogue_index: 0,
        trade_inventory: Some(vec![]),
    });

    // Spawn Additional Barriers
    let max_dist = (world.width.powi(2) + world.height.powi(2)).sqrt() / 2.0;
    let mut extra_barriers = 0;
    let attempts = config.barriers.max_additional_barriers * 10;

    for _ in 0..attempts {
        if extra_barriers >= config.barriers.max_additional_barriers { break; }
        
        let x = rng.gen_range(0.0..world.width);
        let y = rng.gen_range(0.0..world.height);
        let dist = ((x - world.width/2.0).powi(2) + (y - world.height/2.0).powi(2)).sqrt();
        
        let prob = (1.0 - (dist / max_dist)).powi(2) * config.barriers.placement_chance_center;
        
        if rng.gen_bool(prob.clamp(0.0, 1.0)) {
            let id = Uuid::new_v4();
            world.barrier_cores.insert(id, BarrierCore {
                id,
                x,
                y,
                level: 1,
                base_range: config.barriers.base_range,
            });
            
            // 50% chance for villager
            if rng.gen_bool(0.5) {
                world.npcs.insert(Uuid::new_v4(), Npc {
                    id: Uuid::new_v4(),
                    n_type: NpcType::Merchant,
                    name: "Villager".to_string(),
                    x: x + 10.0,
                    y: y + 10.0,
                    health: 100.0,
                    dialogue_index: 0,
                    trade_inventory: Some(vec![]),
                });
            }

            extra_barriers += 1;
        }
    }
    
    for _ in 0..resource_count {
        let id = Uuid::new_v4();
        let r_type = match rng.gen_range(0..3) {
            0 => ResourceType::Tree,
            1 => ResourceType::Rock,
            _ => ResourceType::Food,
        };
        let amount = match r_type {
            ResourceType::Tree => config.balance.resources.tree_amount as f64,
            ResourceType::Rock => config.balance.resources.rock_amount as f64,
            ResourceType::Food => config.balance.resources.food_amount as f64,
        };
        world.resources.insert(id, Resource {
            id,
            r_type,
            x: rng.gen_range(0.0..world.width),
            y: rng.gen_range(0.0..world.height),
            amount,
        });
    }

    for _ in 0..mob_count {
        let id = Uuid::new_v4();
        let m_type = match rng.gen_range(0..3) {
            0 => MobType::Rabbit,
            1 => MobType::Wolf,
            _ => MobType::Bear,
        };
        
        let x = rng.gen_range(0.0..world.width);
        let y = rng.gen_range(0.0..world.height);
        
        // Level scaling
        let dist_from_center = ((x - world.width/2.0).powi(2) + (y - world.height/2.0).powi(2)).sqrt();
        let level = 1 + (dist_from_center * config.leveling.mob_level_factor) as u32;

        let base_health = match m_type {
            MobType::Rabbit => config.balance.mobs.rabbit_health,
            MobType::Wolf => config.balance.mobs.wolf_health,
            _ => config.balance.mobs.bear_health,
        };
        
        // HP multiplier: base * (1 + (level-1)*factor)
        let health = base_health * (1.0 + ((level as f64 - 1.0) * config.balance.mobs.level_hp_mult));

        world.mobs.insert(id, Mob {
            id,
            m_type,
            x,
            y,
            health,
            level,
            target_id: None,
        });
    }
}
