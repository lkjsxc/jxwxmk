use crate::game::entities::item::{Item, ItemType};
use crate::game::entities::player::Inventory;

pub struct Recipe {
    pub output: ItemType,
    pub amount: u32,
    pub ingredients: Vec<(ItemType, u32)>,
}

pub struct CraftingSystem;

impl CraftingSystem {
    pub fn get_recipes() -> Vec<Recipe> {
        vec![
            Recipe {
                output: ItemType::WoodPickaxe,
                amount: 1,
                ingredients: vec![(ItemType::Wood, 10)],
            },
            Recipe {
                output: ItemType::StonePickaxe,
                amount: 1,
                ingredients: vec![(ItemType::Wood, 10), (ItemType::Stone, 10)],
            },
            Recipe {
                output: ItemType::WoodWall,
                amount: 1,
                ingredients: vec![(ItemType::Wood, 20)],
            },
            Recipe {
                output: ItemType::Door,
                amount: 1,
                ingredients: vec![(ItemType::Wood, 30)],
            },
            Recipe {
                output: ItemType::Torch,
                amount: 1,
                ingredients: vec![(ItemType::Wood, 2)],
            },
            Recipe {
                output: ItemType::Workbench,
                amount: 1,
                ingredients: vec![(ItemType::Wood, 50)],
            },
        ]
    }

    pub fn can_craft(inventory: &Inventory, recipe: &Recipe) -> bool {
        for (req_type, req_amt) in &recipe.ingredients {
            let count = inventory.slots.iter().filter_map(|s| {
                s.as_ref().filter(|i| i.kind == *req_type).map(|i| i.amount)
            }).sum::<u32>();
            
            if count < *req_amt {
                return false;
            }
        }
        true
    }

    pub fn craft(inventory: &mut Inventory, output: ItemType) -> bool {
        let recipes = Self::get_recipes();
        if let Some(recipe) = recipes.iter().find(|r| r.output == output) {
            if Self::can_craft(inventory, recipe) {
                // Consume
                for (req_type, req_amt) in &recipe.ingredients {
                    let mut remaining = *req_amt;
                    for slot in inventory.slots.iter_mut() {
                        if let Some(item) = slot {
                            if item.kind == *req_type {
                                let take = remaining.min(item.amount);
                                item.amount -= take;
                                remaining -= take;
                                if item.amount == 0 {
                                    *slot = None;
                                }
                                if remaining == 0 {
                                    break;
                                }
                            }
                        }
                    }
                }
                // Add Output
                inventory.add(recipe.output.clone(), recipe.amount);
                return true;
            }
        }
        false
    }
}
