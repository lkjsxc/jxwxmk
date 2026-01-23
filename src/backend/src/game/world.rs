use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct World {
    pub width: u32,
    pub height: u32,
    pub resources: HashMap<String, Resource>,
}

#[derive(Debug, Clone)]
pub struct Resource {
    pub resource_type: String,
    pub position: (f32, f32),
    pub amount: f32,
}

impl World {
    pub fn new() -> Self {
        World {
            width: 1000,
            height: 1000,
            resources: HashMap::new(),
        }
    }
}