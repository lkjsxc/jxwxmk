use crate::config::Config;
use crate::game::world::entities::{Item, PlayerState};

pub fn craft(player: &mut PlayerState, config: &Config, recipe_id: &str) -> bool {
    let recipe = match config.crafting.recipes.iter().find(|r| r.id == recipe_id) {
        Some(recipe) => recipe,
        None => return false,
    };

    for ingredient in &recipe.inputs {
        if !player.inventory.has_item(&ingredient.item, ingredient.count) {
            return false;
        }
    }

    for ingredient in &recipe.inputs {
        player.inventory.consume_item(&ingredient.item, ingredient.count);
    }

    let item = Item::new(recipe.output.item.clone(), recipe.output.count);
    player.inventory.add_item(item);
    player.stats.crafts += 1;
    player.xp += 5;
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use uuid::Uuid;

    #[test]
    fn crafting_consumes_items() {
        let config = Config::default();
        let mut player = PlayerState::new(Uuid::new_v4(), Uuid::new_v4(), 10);
        player.inventory.add_item(Item::new("Wood", 10));
        let ok = craft(&mut player, &config, "WoodPickaxe");
        assert!(ok);
        assert!(!player.inventory.has_item("Wood", 1));
    }

}
