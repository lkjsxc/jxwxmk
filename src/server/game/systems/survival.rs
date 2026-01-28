use crate::game::world::World;

pub struct SurvivalSystem;

impl SurvivalSystem {
    pub fn update(world: &mut World, dt: f64) {
        let hunger_decay = 1.0; // TODO: Load from config
        let starve_damage = 5.0;
        let heal_rate = 2.0;
        let heal_threshold = 80.0;

        for player in world.players.values_mut() {
            if !player.spawned { continue; }

            // Hunger Decay
            // TODO: Biome modifier
            player.hunger = (player.hunger - hunger_decay * dt).max(0.0);

            // Health effects
            if player.hunger <= 0.0 {
                player.health = (player.health - starve_damage * dt).max(0.0);
            } else if player.hunger > heal_threshold && player.health < player.max_health {
                player.health = (player.health + heal_rate * dt).min(player.max_health);
            }

            // Temperature (Simple lerp to neutral for now, placeholder)
            // let neutral_temp = 20.0;
            // let converge_rate = 0.1;
            // player.temperature = player.temperature + (neutral_temp - player.temperature) * converge_rate * dt;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::world::{World, PlayerState};
    use uuid::Uuid;

    #[test]
    fn test_hunger_decay() {
        let mut world = World::new();
        let id = Uuid::new_v4();
        world.players.insert(id, PlayerState {
            id,
            token: Uuid::new_v4(),
            username: "Test".to_string(),
            x: 0.0, y: 0.0,
            health: 100.0, max_health: 100.0,
            hunger: 100.0, // Start full
            temperature: 20.0,
            spawned: true,
        });

        SurvivalSystem::update(&mut world, 1.0); // 1 sec
        
        let p = world.players.get(&id).unwrap();
        assert!(p.hunger < 100.0);
        assert_eq!(p.health, 100.0); // No damage yet
    }

    #[test]
    fn test_starvation() {
        let mut world = World::new();
        let id = Uuid::new_v4();
        world.players.insert(id, PlayerState {
            id,
            token: Uuid::new_v4(),
            username: "Test".to_string(),
            x: 0.0, y: 0.0,
            health: 100.0, max_health: 100.0,
            hunger: 0.0, // Starving
            temperature: 20.0,
            spawned: true,
        });

        SurvivalSystem::update(&mut world, 1.0);
        
        let p = world.players.get(&id).unwrap();
        assert!(p.health < 100.0);
    }
}
