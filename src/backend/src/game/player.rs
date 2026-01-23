use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub position: (f32, f32),
    pub health: f32,
    pub hunger: f32,
    pub thirst: f32,
    pub inventory: Vec<String>,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player {
            id: Uuid::new_v4(),
            name,
            position: (0.0, 0.0),
            health: 100.0,
            hunger: 100.0,
            thirst: 100.0,
            inventory: Vec::new(),
        }
    }
}