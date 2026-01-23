pub mod player;
pub mod world;
pub mod items;
pub mod crafting;

pub struct GameState {
    pub players: Vec<player::Player>,
    pub world: world::World,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            players: Vec::new(),
            world: world::World::new(),
        }
    }
}