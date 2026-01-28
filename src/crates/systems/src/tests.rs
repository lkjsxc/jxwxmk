#[cfg(test)]
mod tests {
    use super::*;
    use world::{World, Vec2, ChunkCoord};
    use config::{Config, Recipe, RecipeInput, RecipeOutput};
    use std::time::Duration;
    use crate::survival;
    use crate::crafting;

    struct MockMsg(String);
    impl actix::Message for MockMsg { type Result = (); }
    impl From<String> for MockMsg { fn from(s: String) -> Self { MockMsg(s) } }

    fn create_test_world() -> World {
        let mut config = Config::default();
        config.world.chunk_size_wu = 128;
        World::new(config)
    }

    #[test]
    fn test_crafting_logic() {
        let mut world = create_test_world();
        
        // Add recipe to config
        world.config.crafting.recipes.push(Recipe {
            id: "wood_pickaxe".into(),
            station: "hand".into(),
            inputs: vec![RecipeInput { item: "wood".into(), count: 10 }],
            output: RecipeOutput { item: "wood_pickaxe".into(), count: 1 },
        });

        let pid = uuid::Uuid::new_v4();
        let mut player = world::PlayerState {
            id: pid,
            token: uuid::Uuid::new_v4(),
            name: "Test".into(),
            level: 1,
            xp: 0,
            pos: Vec2 { x: 0.0, y: 0.0 },
            chunk: ChunkCoord { x: 0, y: 0 },
            hp: 100.0,
            max_hp: 100.0,
            hunger: 100.0,
            thirst: 100.0,
            temp: 50.0,
            inventory: vec![None; 28],
            active_slot: 0,
            stats: std::collections::HashMap::new(),
            unlocked_achievements: std::collections::HashSet::new(),
            stat_bonuses: std::collections::HashMap::new(),
            active_quests: Vec::new(),
            spawned: true,
            active_view: std::collections::HashSet::new(),
            input_dx: 0.0,
            input_dy: 0.0,
            input_attack: false,
            input_interact: false,
            input_aim: None,
        };

        player.inventory[0] = Some(world::InventorySlot { item_id: "wood".into(), count: 10 });
        world.players.insert(pid, player);

        let result = crafting::craft(&mut world, pid, "wood_pickaxe");
        assert!(result.is_ok());
        
        let p = world.players.get(&pid).unwrap();
        // Wood should be gone
        let wood_count: u32 = p.inventory.iter().flatten().filter(|s| s.item_id == "wood").map(|s| s.count).sum();
        assert_eq!(wood_count, 0);
        // Pickaxe should be present
        let has_pickaxe = p.inventory.iter().flatten().any(|s| s.item_id == "wood_pickaxe" && s.count == 1);
        assert!(has_pickaxe);
    }

    #[test]
    fn test_survival_decay() {
        let mut world = create_test_world();
        let pid = uuid::Uuid::new_v4();
        world.players.insert(pid, world::PlayerState {
            id: pid,
            token: uuid::Uuid::new_v4(),
            name: "Test".into(),
            level: 1,
            xp: 0,
            pos: Vec2 { x: 0.0, y: 0.0 },
            chunk: ChunkCoord { x: 0, y: 0 },
            hp: 100.0,
            max_hp: 100.0,
            hunger: 100.0,
            thirst: 100.0,
            temp: 50.0,
            inventory: vec![None; 28],
            active_slot: 0,
            stats: std::collections::HashMap::new(),
            unlocked_achievements: std::collections::HashSet::new(),
            stat_bonuses: std::collections::HashMap::new(),
            active_quests: Vec::new(),
            spawned: true,
            active_view: std::collections::HashSet::new(),
            input_dx: 0.0,
            input_dy: 0.0,
            input_attack: false,
            input_interact: false,
            input_aim: None,
        });

        survival::tick(&mut world, Duration::from_secs(1));

        let p = world.players.get(&pid).unwrap();
        assert!(p.hunger < 100.0);
    }
}