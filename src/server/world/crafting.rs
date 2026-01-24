use crate::world::Player;
use std::collections::HashMap;

pub fn craft_item(player: &mut Player, data: Vec<u8>) {
    // Placeholder: data is recipe_id
    let recipe_id = data.get(0).copied().unwrap_or(0);
    let recipe = get_recipe(recipe_id);
    if can_craft(player, &recipe) {
        for (item_id, qty) in &recipe.inputs {
            if let Some(inv_qty) = player.inventory.get_mut(item_id) {
                *inv_qty -= qty;
            }
        }
        *player.inventory.entry(recipe.output).or_insert(0) += 1;
    }
}

fn can_craft(player: &Player, recipe: &Recipe) -> bool {
    for (item_id, qty) in &recipe.inputs {
        if player.inventory.get(item_id).unwrap_or(&0) < qty {
            return false;
        }
    }
    true
}

struct Recipe {
    inputs: HashMap<u32, u32>,
    output: u32,
}

fn get_recipe(id: u8) -> Recipe {
    match id {
        0 => Recipe { inputs: [(0, 2)].into(), output: 2 },  // 2 wood -> pickaxe
        _ => Recipe { inputs: HashMap::new(), output: 0 },
    }
}