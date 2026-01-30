use config::BalanceConfig;
use protocol::{InputData, Aim};
use world::{PlayerState, World, Entity, EntityKind};

pub struct InteractionSystem;

impl InteractionSystem {
    pub fn handle_input(
        player: &mut PlayerState,
        input: &InputData,
        world: &World,
        balance: &BalanceConfig,
        dt: f64,
    ) {
        if !player.spawned {
            return;
        }

        // Movement
        let speed = balance.player.base_speed;
        player.move_by(input.dx, input.dy, speed, dt);

        // Actions (if aim is provided)
        if let Some(ref aim) = input.aim {
            if input.interact {
                Self::handle_interact(player, aim, world, balance);
            }
            if input.attack {
                Self::handle_attack(player, aim, world, balance);
            }
        }
    }

    fn handle_interact(player: &mut PlayerState, aim: &Aim, world: &World, balance: &BalanceConfig) {
        let range = balance.player.interaction_range_wu;
        
        // Check for resources to gather
        if let Some(resource) = Self::find_resource_in_range(player, aim, world, range) {
            // Gathering logic would go here
            player.stats.gathers += 1;
            
            // Add some wood for testing
            player.add_inventory_item("wood".to_string(), 1);
        }
    }

    fn handle_attack(player: &mut PlayerState, aim: &Aim, world: &World, balance: &BalanceConfig) {
        let range = balance.player.interaction_range_wu;
        
        // Check for mobs to attack
        if let Some(_mob) = Self::find_mob_in_range(player, aim, world, range) {
            // Combat logic would go here
            player.stats.kills += 1;
        }
    }

    fn find_resource_in_range<'a>(
        player: &PlayerState,
        aim: &Aim,
        world: &'a World,
        range: f64,
    ) -> Option<&'a Entity> {
        let (cx, cy) = world.world_to_chunk(player.x, player.y);
        
        world.get_chunk((cx, cy)).and_then(|chunk| {
            chunk.resources.iter().min_by(|a, b| {
                let da = a.distance_to_point(aim.x, aim.y);
                let db = b.distance_to_point(aim.x, aim.y);
                da.partial_cmp(&db).unwrap()
            }).filter(|e| e.distance_to_point(aim.x, aim.y) <= range)
        })
    }

    fn find_mob_in_range<'a>(
        player: &PlayerState,
        aim: &Aim,
        world: &'a World,
        range: f64,
    ) -> Option<&'a Entity> {
        let (cx, cy) = world.world_to_chunk(player.x, player.y);
        
        world.get_chunk((cx, cy)).and_then(|chunk| {
            chunk.mobs.iter().min_by(|a, b| {
                let da = a.distance_to_point(aim.x, aim.y);
                let db = b.distance_to_point(aim.x, aim.y);
                da.partial_cmp(&db).unwrap()
            }).filter(|e| e.distance_to_point(aim.x, aim.y) <= range)
        })
    }

    pub fn handle_slot_change(player: &mut PlayerState, slot: usize, max_slots: usize) {
        if slot < max_slots {
            player.active_slot = slot;
        }
    }

    pub fn handle_swap_slots(player: &mut PlayerState, from: usize, to: usize, max_slots: usize) {
        if from < max_slots && to < max_slots {
            player.inventory.swap(from, to);
        }
    }
}
