use crate::config::Config;
use crate::game::entities::{ItemId, PlayerId};
use crate::game::world::World;

pub fn handle_craft(world: &mut World, config: &Config, player_id: PlayerId, recipe_id: &str) -> bool {
    let Some(recipe) = config.crafting.recipes.iter().find(|r| r.id == recipe_id) else {
        return false;
    };

    let Some(player) = world.get_player_mut(&player_id) else {
        return false;
    };

    for input in &recipe.inputs {
        let item_id = ItemId::new(input.item.clone());
        if player.inventory.item_count(&item_id) < input.count {
            return false;
        }
    }

    for input in &recipe.inputs {
        let item_id = ItemId::new(input.item.clone());
        if !player.inventory.remove_item(&item_id, input.count) {
            return false;
        }
    }

    let output_id = ItemId::new(recipe.output.item.clone());
    let added = player.inventory.add_item(output_id, recipe.output.count);
    if added {
        player.stats.crafts = player.stats.crafts.saturating_add(1);
    }
    added
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{
        AchievementsConfig, BalanceConfig, BiomesConfig, Config, CraftingConfig, EconomyConfig,
        QuestsConfig, RecipeDef, RecipeInput, RecipeOutput, ServerConfig, SettlementsConfig,
        SpawningConfig, SurvivalConfig, WorldConfig,
    };
    use crate::game::entities::{ItemId, PlayerState};
    use crate::game::world::World;
    use uuid::Uuid;

    fn base_config() -> Config {
        let recipe = RecipeDef {
            id: "Test".to_string(),
            station_tier: 0,
            inputs: vec![RecipeInput {
                item: "Wood".to_string(),
                count: 2,
            }],
            output: RecipeOutput {
                item: "Torch".to_string(),
                count: 1,
            },
        };
        Config::new(
            ServerConfig::default(),
            WorldConfig::default(),
            BalanceConfig::default(),
            SurvivalConfig::default(),
            CraftingConfig { recipes: vec![recipe] },
            SpawningConfig::default(),
            BiomesConfig::default(),
            SettlementsConfig::default(),
            EconomyConfig::default(),
            QuestsConfig::default(),
            AchievementsConfig::default(),
        )
    }

    #[test]
    fn crafts_when_materials_present() {
        let config = base_config();
        let mut world = World::new(0);
        let mut player = PlayerState::new(Uuid::new_v4(), Uuid::new_v4(), 5, 100.0);
        player.inventory.add_item(ItemId::new("Wood"), 2);
        world.upsert_player(player);

        let player_id = *world.players.keys().next().unwrap();
        let crafted = handle_craft(&mut world, &config, player_id, "Test");
        assert!(crafted);
        let player = world.get_player(&player_id).unwrap();
        assert_eq!(player.inventory.item_count(&ItemId::new("Wood")), 0);
        assert_eq!(player.inventory.item_count(&ItemId::new("Torch")), 1);
    }
}
