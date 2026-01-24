use crate::world::Player;

pub fn update_survival(player: &mut Player) {
    player.hunger -= 0.1;
    if player.hunger <= 0.0 {
        player.health -= 0.5;  // Starvation damage
    }
    // Clamp
    player.hunger = player.hunger.max(0.0).min(100.0);
    player.health = player.health.max(0.0).min(100.0);
}

pub fn consume_food(player: &mut Player, data: Vec<u8>) {
    // Placeholder: data is food_id
    let food_id = data.get(0).copied().unwrap_or(0) as u32;
    if let Some(qty) = player.inventory.get_mut(&food_id) {
        if *qty > 0 {
            *qty -= 1;
            player.hunger += 20.0;  // Restore hunger
        }
    }
}