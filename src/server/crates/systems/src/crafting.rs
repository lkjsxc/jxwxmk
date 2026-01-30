use config::{CraftingConfig, Recipe};
use world::PlayerState;

pub struct CraftingSystem;

impl CraftingSystem {
    pub fn try_craft(player: &mut PlayerState, recipe_id: &str, config: &CraftingConfig) -> bool {
        // Find the recipe
        let recipe = match config.recipes.iter().find(|r| r.id == recipe_id) {
            Some(r) => r,
            None => return false,
        };

        // Check if player has all required inputs
        for input in &recipe.inputs {
            if !player.has_item(&input.item, input.count) {
                return false;
            }
        }

        // Consume inputs
        for input in &recipe.inputs {
            player.remove_inventory_item(&input.item, input.count);
        }

        // Give output
        player.add_inventory_item(recipe.output.item.clone(), recipe.output.count);
        
        // Update stats
        player.stats.crafts += 1;

        true
    }

    pub fn get_available_recipes<'a>(player: &PlayerState, config: &'a CraftingConfig) -> Vec<&'a Recipe> {
        config.recipes.iter().filter(|recipe| {
            recipe.inputs.iter().all(|input| {
                player.has_item(&input.item, input.count)
            })
        }).collect()
    }
}
