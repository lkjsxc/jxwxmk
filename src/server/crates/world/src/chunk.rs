use crate::entity::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub coord: (i32, i32),
    pub biome_id: String,
    pub resources: Vec<Entity>,
    pub mobs: Vec<Entity>,
    pub structures: Vec<Entity>,
    pub npcs: Vec<Entity>,
    pub respawn_cooldowns: HashMap<String, f64>,
    pub dirty: bool,
}

impl Chunk {
    pub fn generate(coord: (i32, i32), world_seed: u64, chunk_size: i32) -> Self {
        // Deterministic biome selection based on coord + seed
        let biome_id = Self::select_biome(coord, world_seed);
        
        let mut chunk = Self {
            coord,
            biome_id,
            resources: Vec::new(),
            mobs: Vec::new(),
            structures: Vec::new(),
            npcs: Vec::new(),
            respawn_cooldowns: HashMap::new(),
            dirty: false,
        };
        
        // Generate baseline resources
        chunk.generate_resources(world_seed, chunk_size);
        
        chunk
    }

    fn select_biome(coord: (i32, i32), seed: u64) -> String {
        // Simple deterministic biome selection
        let hash = Self::coord_hash(coord, seed);
        if hash % 3 == 0 {
            "forest".to_string()
        } else if hash % 3 == 1 {
            "plains".to_string()
        } else {
            "hills".to_string()
        }
    }

    fn coord_hash(coord: (i32, i32), seed: u64) -> u64 {
        let (x, y) = coord;
        let mut h = seed;
        h = h.wrapping_mul(31).wrapping_add(x as u64);
        h = h.wrapping_mul(31).wrapping_add(y as u64);
        h
    }

    fn generate_resources(&mut self, world_seed: u64, chunk_size: i32) {
        let base_x = self.coord.0 as f64 * chunk_size as f64;
        let base_y = self.coord.1 as f64 * chunk_size as f64;
        
        // Generate some trees
        let hash = Self::coord_hash(self.coord, world_seed);
        let num_trees = ((hash % 15) + 5) as usize;
        
        for i in 0..num_trees {
            let tree_hash = hash.wrapping_mul(17).wrapping_add(i as u64);
            let x = base_x + ((tree_hash % chunk_size as u64) as f64);
            let y = base_y + (((tree_hash / 100) % chunk_size as u64) as f64);
            
            self.resources.push(Entity::new(
                format!("tree_{}_{}_{}", self.coord.0, self.coord.1, i),
                EntityKind::Resource,
                "tree".to_string(),
                x,
                y,
                30.0,
            ));
        }
        
        // Generate some rocks
        let num_rocks = ((hash % 8) + 3) as usize;
        for i in 0..num_rocks {
            let rock_hash = hash.wrapping_mul(23).wrapping_add(i as u64);
            let x = base_x + ((rock_hash % chunk_size as u64) as f64);
            let y = base_y + (((rock_hash / 200) % chunk_size as u64) as f64);
            
            self.resources.push(Entity::new(
                format!("rock_{}_{}_{}", self.coord.0, self.coord.1, i),
                EntityKind::Resource,
                "rock".to_string(),
                x,
                y,
                30.0,
            ));
        }
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }
}
