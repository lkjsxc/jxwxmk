use crate::world::{Player, ResourceNode};

pub fn gather_resource(player: &mut Player, resources: &mut Vec<ResourceNode>, data: Vec<u8>) {
    // Placeholder: assume data is node_id
    let node_id = data.get(0).copied().unwrap_or(0) as u32;
    if let Some(node) = resources.iter_mut().find(|n| n.id == node_id && !n.depleted) {
        // Check distance
        let dist = ((player.position.0 - node.position.0).powi(2) + (player.position.1 - node.position.1).powi(2)).sqrt();
        if dist < 10.0 {  // Within range
            let item_id = match node.node_type.as_str() {
                "tree" => 0,  // wood
                "rock" => 1,  // stone
                _ => return,
            };
            *player.inventory.entry(item_id).or_insert(0) += 1;
            node.depleted = true;
            node.respawn_tick = 100;  // Respawn in 100 ticks
        }
    }
}

pub fn update_resources(resources: &mut Vec<ResourceNode>, current_tick: u64) {
    for node in resources.iter_mut() {
        if node.depleted && current_tick >= node.respawn_tick {
            node.depleted = false;
        }
    }
}