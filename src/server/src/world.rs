use std::collections::HashMap;
use tracing::{debug, info};

pub struct GameWorld {
    width: u32,
    height: u32,
    resources: HashMap<String, ResourceNode>,
    entities: HashMap<String, WorldEntity>,
    chunks: HashMap<(i32, i32), WorldChunk>,
}

#[derive(Debug, Clone)]
pub struct ResourceNode {
    pub id: String,
    pub resource_type: ResourceType,
    pub position: (f32, f32),
    pub quantity: f32,
    pub max_quantity: f32,
    pub respawn_time: Option<f64>,
    pub biome: BiomeType,
}

#[derive(Debug, Clone)]
pub struct WorldEntity {
    pub id: String,
    pub entity_type: EntityType,
    pub position: (f32, f32),
    pub velocity: (f32, f32),
    pub health: f32,
    pub max_health: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResourceType {
    Tree,
    Rock,
    Bush,
    Ore,
    Water,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EntityType {
    Player,
    Animal,
    Monster,
    NPC,
    Projectile,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BiomeType {
    Forest,
    Desert,
    Mountain,
    Ocean,
    Grassland,
}

#[derive(Debug, Clone)]
pub struct WorldChunk {
    pub x: i32,
    pub y: i32,
    pub resources: Vec<String>,
    pub entities: Vec<String>,
}

impl GameWorld {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            resources: HashMap::new(),
            entities: HashMap::new(),
            chunks: HashMap::new(),
        }
    }
    
    pub fn add_resource(&mut self, resource: ResourceNode) {
        let resource_id = resource.id.clone();
        let chunk_pos = self.calculate_chunk_position(resource.position.0, resource.position.1);
        
        self.resources.insert(resource_id.clone(), resource);
        
        let chunk = self.chunks.entry(chunk_pos).or_insert_with(|| WorldChunk {
            x: chunk_pos.0,
            y: chunk_pos.1,
            resources: Vec::new(),
            entities: Vec::new(),
        });
        
        chunk.resources.push(resource_id);
    }
    
    pub fn add_entity(&mut self, entity: WorldEntity) {
        let entity_id = entity.id.clone();
        let chunk_pos = self.calculate_chunk_position(entity.position.0, entity.position.1);
        
        self.entities.insert(entity_id.clone(), entity);
        
        let chunk = self.chunks.entry(chunk_pos).or_insert_with(|| WorldChunk {
            x: chunk_pos.0,
            y: chunk_pos.1,
            resources: Vec::new(),
            entities: Vec::new(),
        });
        
        chunk.entities.push(entity_id);
    }
    
    pub fn remove_resource(&mut self, resource_id: &str) -> Option<ResourceNode> {
        if let Some(resource) = self.resources.remove(resource_id) {
            let chunk_pos = self.calculate_chunk_position(resource.position.0, resource.position.1);
            
            if let Some(chunk) = self.chunks.get_mut(&chunk_pos) {
                chunk.resources.retain(|id| id != resource_id);
            }
            
            Some(resource)
        } else {
            None
        }
    }
    
    pub fn get_resources_in_area(&self, x: f32, y: f32, radius: f32) -> Vec<&ResourceNode> {
        let chunk_pos = self.calculate_chunk_position(x, y);
        let mut resources = Vec::new();
        
        // Check current chunk and neighboring chunks
        for dx in -1..=1 {
            for dy in -1..=1 {
                let check_chunk_pos = (chunk_pos.0 + dx, chunk_pos.1 + dy);
                if let Some(chunk) = self.chunks.get(&check_chunk_pos) {
                    for resource_id in &chunk.resources {
                        if let Some(resource) = self.resources.get(resource_id) {
                            let distance = ((resource.position.0 - x).powi(2) + (resource.position.1 - y).powi(2)).sqrt();
                            if distance <= radius {
                                resources.push(resource);
                            }
                        }
                    }
                }
            }
        }
        
        resources
    }
    
    fn calculate_chunk_position(&self, x: f32, y: f32) -> (i32, i32) {
        const CHUNK_SIZE: f32 = 100.0;
        (x.div_euclid(CHUNK_SIZE) as i32, y.div_euclid(CHUNK_SIZE) as i32)
    }
    
    pub fn update(&mut self, delta_time: f32) {
        // Update resource respawns
        for resource in self.resources.values_mut() {
            if let Some(respawn_time) = resource.respawn_time {
                // In a real implementation, this would check if respawn time has passed
                // and restore the resource quantity
            }
        }
        
        // Update entity positions
        for entity in self.entities.values_mut() {
            entity.position.0 += entity.velocity.0 * delta_time;
            entity.position.1 += entity.velocity.1 * delta_time;
        }
    }
}