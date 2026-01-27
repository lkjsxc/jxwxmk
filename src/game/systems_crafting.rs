use crate::game::world_state::{Player, Item, ItemType, Inventory};
use std::collections::HashMap;

pub struct Recipe {
    pub input: Vec<(ItemType, u32)>,
    pub output: (ItemType, u32),
}

pub fn get_recipes() -> HashMap<ItemType, Recipe> {
    let mut map = HashMap::new();
    
    map.insert(ItemType::WoodPickaxe, Recipe {
        input: vec![(ItemType::Wood, 10)],
        output: (ItemType::WoodPickaxe, 1),
    });
    
    map.insert(ItemType::StonePickaxe, Recipe {
        input: vec![(ItemType::Wood, 10), (ItemType::Stone, 10)],
        output: (ItemType::StonePickaxe, 1),
    });
    
    map.insert(ItemType::WoodWall, Recipe {
        input: vec![(ItemType::Wood, 20)],
        output: (ItemType::WoodWall, 1),
    });
    
    map.insert(ItemType::Door, Recipe {
        input: vec![(ItemType::Wood, 30)],
        output: (ItemType::Door, 1),
    });
    
    map.insert(ItemType::Torch, Recipe {
        input: vec![(ItemType::Wood, 2)],
        output: (ItemType::Torch, 1),
    });
    
    map.insert(ItemType::Workbench, Recipe {
        input: vec![(ItemType::Wood, 50)],
        output: (ItemType::Workbench, 1),
    });

    map
}

pub fn handle_craft(player: &mut Player, item_type: ItemType) -> bool {
    let recipes = get_recipes();
    
    if let Some(recipe) = recipes.get(&item_type) {
        // Check availability
        for (req_type, req_amount) in &recipe.input {
            if count_item(&player.inventory, *req_type) < *req_amount {
                return false;
            }
        }

        // Consume items
        for (req_type, req_amount) in &recipe.input {
            remove_item(&mut player.inventory, *req_type, *req_amount);
        }

        // Add output
        add_item(&mut player.inventory, recipe.output.0, recipe.output.1);
        player.stats.crafts += 1;
        return true;
    }

    false
}

fn count_item(inv: &Inventory, kind: ItemType) -> u32 {
    let mut total = 0;
    for slot in &inv.slots {
        if let Some(item) = slot {
            if item.kind == kind {
                total += item.amount;
            }
        }
    }
    total
}

fn remove_item(inv: &mut Inventory, kind: ItemType, mut amount: u32) {
    for slot in inv.slots.iter_mut() {
        if amount == 0 { break; }
        if let Some(item) = slot {
            if item.kind == kind {
                if item.amount > amount {
                    item.amount -= amount;
                    amount = 0;
                } else {
                    amount -= item.amount;
                    *slot = None;
                }
            }
        }
    }
}

pub fn add_item(inv: &mut Inventory, kind: ItemType, mut amount: u32) {
    // Stack first
    for slot in inv.slots.iter_mut() {
        if amount == 0 { return; }
        if let Some(item) = slot {
            if item.kind == kind { // Unlimited stack for now
                item.amount += amount;
                amount = 0;
            }
        }
    }

    // Fill empty slots
    for slot in inv.slots.iter_mut() {
        if amount == 0 { return; }
        if slot.is_none() {
            *slot = Some(Item {
                kind,
                amount,
                max_stack: u32::MAX,
                level: 1,
                xp: 0.0,
            });
            amount = 0;
        }
    }
}
