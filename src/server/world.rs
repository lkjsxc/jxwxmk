use std::collections::HashMap;
use tokio::sync::mpsc;

pub mod gathering;
pub mod crafting;
pub mod survival;
pub mod combat;

#[derive(Clone, Debug)]
pub struct Player {
    pub id: String,
    pub position: (f32, f32),
    pub health: f32,
    pub hunger: f32,
    pub inventory: HashMap<u32, u32>,
}

#[derive(Clone, Debug)]
pub struct ResourceNode {
    pub id: u32,
    pub node_type: String,
    pub position: (f32, f32),
    pub depleted: bool,
    pub respawn_tick: u64,
}

#[derive(Clone)]
pub struct WorldState {
    pub players: HashMap<String, Player>,
    pub resources: Vec<ResourceNode>,
    pub current_tick: u64,
}

impl WorldState {
    pub fn new() -> Self {
        WorldState {
            players: HashMap::new(),
            resources: vec![
                ResourceNode {
                    id: 1,
                    node_type: "tree".to_string(),
                    position: (10.0, 10.0),
                    depleted: false,
                    respawn_tick: 0,
                },
                ResourceNode {
                    id: 2,
                    node_type: "rock".to_string(),
                    position: (20.0, 20.0),
                    depleted: false,
                    respawn_tick: 0,
                },
            ],
            current_tick: 0,
        }
    }
}

pub async fn run_simulation(
    mut input_rx: mpsc::UnboundedReceiver<InputEvent>,
    snapshot_tx: mpsc::UnboundedSender<SnapshotEvent>,
    mut world: WorldState,
) {
    let mut interval = tokio::time::interval(std::time::Duration::from_millis(50)); // 20 Hz

    loop {
        interval.tick().await;

        // Process inputs
        while let Ok(input) = input_rx.try_recv() {
            match input {
                InputEvent::Move { player_id, x, y } => {
                    if let Some(player) = world.players.get_mut(&player_id) {
                        player.position = (x, y);
                    }
                }
                InputEvent::Gather { player_id, node_id } => {
                    if let Some(player) = world.players.get_mut(&player_id) {
                        gathering::gather_resource(player, &mut world.resources, vec![node_id as u8]);
                    }
                }
                InputEvent::Craft { player_id, recipe_id } => {
                    if let Some(player) = world.players.get_mut(&player_id) {
                        crafting::craft_item(player, vec![recipe_id]);
                    }
                }
                InputEvent::Consume { player_id, food_id } => {
                    if let Some(player) = world.players.get_mut(&player_id) {
                        survival::consume_food(player, vec![food_id as u8]);
                    }
                }
                InputEvent::Join { player_id } => {
                    let player = Player {
                        id: player_id.clone(),
                        position: (0.0, 0.0),
                        health: 100.0,
                        hunger: 100.0,
                        inventory: HashMap::new(),
                    };
                    world.players.insert(player_id, player);
                }
            }
        }

        // Update survival
        for player in world.players.values_mut() {
            survival::update_survival(player);
        }

        // Update resources
        gathering::update_resources(&mut world.resources, world.current_tick);

        // Send snapshot
        let snapshot = SnapshotEvent {
            tick: world.current_tick,
            players: world.players.clone(),
            resources: world.resources.clone(),
        };
        let _ = snapshot_tx.send(snapshot);

        world.current_tick += 1;
    }
}

#[derive(Debug)]
pub enum InputEvent {
    Move { player_id: String, x: f32, y: f32 },
    Gather { player_id: String, node_id: u32 },
    Craft { player_id: String, recipe_id: u8 },
    Consume { player_id: String, food_id: u8 },
    Join { player_id: String },
}

#[derive(Clone, Debug)]
pub struct SnapshotEvent {
    pub tick: u64,
    pub players: HashMap<String, Player>,
    pub resources: Vec<ResourceNode>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_gather_resource() {
        let mut player = Player {
            id: "test".to_string(),
            position: (0.0, 0.0),
            health: 100.0,
            hunger: 100.0,
            inventory: HashMap::new(),
        };
        let mut resources = vec![ResourceNode {
            id: 1,
            node_type: "tree".to_string(),
            position: (5.0, 5.0),
            depleted: false,
            respawn_tick: 0,
        }];
        gathering::gather_resource(&mut player, &mut resources, vec![1]);
        assert_eq!(player.inventory.get(&0), Some(&1));  // wood
        assert!(resources[0].depleted);
    }

    #[test]
    fn test_craft_item() {
        let mut player = Player {
            id: "test".to_string(),
            position: (0.0, 0.0),
            health: 100.0,
            hunger: 100.0,
            inventory: [(0, 2)].into(),  // 2 wood
        };
        crafting::craft_item(&mut player, vec![0]);
        assert_eq!(player.inventory.get(&0), Some(&0));  // wood consumed
        assert_eq!(player.inventory.get(&2), Some(&1));  // pickaxe
    }
}