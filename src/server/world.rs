#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_gather_resource() {
        let mut player = Player {
            id: "test".to_string(),
            position: (0.0, 0.0),
            health: 100.0,
            hunger: 100.0,
            inventory: HashMap::new(),
        };
        let mut resources = vec![ResourceNode {
            id: 1,
            node_type: "tree".to_string(),
            position: (5.0, 5.0),
            depleted: false,
            respawn_tick: 0,
        }];
        gathering::gather_resource(&mut player, &mut resources, vec![1]);
        assert_eq!(player.inventory.get(&0), Some(&1));  // wood
        assert!(resources[0].depleted);
    }

    #[test]
    fn test_craft_item() {
        let mut player = Player {
            id: "test".to_string(),
            position: (0.0, 0.0),
            health: 100.0,
            hunger: 100.0,
            inventory: [(0, 2)].into(),  // 2 wood
        };
        crafting::craft_item(&mut player, vec![0]);
        assert_eq!(player.inventory.get(&0), Some(&0));  // wood consumed
        assert_eq!(player.inventory.get(&2), Some(&1));  // pickaxe
    }
}