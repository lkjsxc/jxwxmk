use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
}

impl PartialEq for ChunkCoord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for ChunkCoord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl ChunkCoord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
