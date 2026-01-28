use world::{World, InventorySlot};
use uuid::Uuid;

pub fn craft(world: &mut World, player_id: Uuid, recipe_id: &str) -> Result<(), String> {
    let recipes = world.config.crafting.recipes.clone();
    let recipe = recipes.iter().find(|r| r.id == recipe_id)
        .ok_or_else(|| "Recipe not found".to_string())?;

    let player = world.players.get_mut(&player_id)
        .ok_or_else(|| "Player not found".to_string())?;

    // 1. Check ingredients
    for input in &recipe.inputs {
        let has_count: u32 = player.inventory.iter()
            .flatten()
            .filter(|slot| slot.item_id == input.item)
            .map(|slot| slot.count)
            .sum();
        if has_count < input.count {
            return Err("Insufficient items".to_string());
        }
    }

    // 2. Consume ingredients
    for input in &recipe.inputs {
        let mut remaining = input.count;
        for slot in player.inventory.iter_mut() {
            if let Some(s) = slot {
                if s.item_id == input.item {
                    let to_take = s.count.min(remaining);
                    s.count -= to_take;
                    remaining -= to_take;
                    if s.count == 0 {
                        *slot = None;
                    }
                }
            }
            if remaining == 0 { break; }
        }
    }

    // 3. Add output
    add_to_inventory(player, &recipe.output.item, recipe.output.count);

    player.xp += 10; // Base crafting XP
    *player.stats.entry("items_crafted".to_string()).or_insert(0.0) += 1.0;

    Ok(())
}

fn add_to_inventory(player: &mut world::PlayerState, item_id: &str, count: u32) {
    // Try to stack
    for slot in player.inventory.iter_mut().flatten() {
        if slot.item_id == item_id {
            slot.count += count;
            return;
        }
    }
    // Try to find empty slot
    for slot in player.inventory.iter_mut() {
        if slot.is_none() {
            *slot = Some(InventorySlot {
                item_id: item_id.to_string(),
                count,
            });
            return;
        }
    }
    // If full, we'd drop it. For now, just append if we were using a Vec, but it's fixed size.
    // In world.rs, inventory is Vec<Option<InventorySlot>>.
}
