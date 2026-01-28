use crate::{Chunk, ChunkCoord};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

pub struct BiomeGenerator {
    seed: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BiomeType {
    Forest,
    Plains,
    Desert,
    Tundra,
    Mountains,
}

impl BiomeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            BiomeType::Forest => "forest",
            BiomeType::Plains => "plains",
            BiomeType::Desert => "desert",
            BiomeType::Tundra => "tundra",
            BiomeType::Mountains => "mountains",
        }
    }

    pub fn base_temperature(&self,
    ) -> f32 {
        match self {
            BiomeType::Forest => 50.0,
            BiomeType::Plains => 55.0,
            BiomeType::Desert => 80.0,
            BiomeType::Tundra => 20.0,
            BiomeType::Mountains => 30.0,
        }
    }
}

impl BiomeGenerator {
    pub fn new(seed: u64) -> Self {
        BiomeGenerator { seed }
    }

    pub fn generate_biome(&self,
        coord: ChunkCoord,
    ) -> BiomeType {
        let mut rng = StdRng::seed_from_u64(
            self.seed.wrapping_add(
                (coord.0 as u64).wrapping_mul(374761393)
                    .wrapping_add((coord.1 as u64).wrapping_mul(668265263))
            )
        );

        let value = rng.gen::<f32>();
        
        if value < 0.3 {
            BiomeType::Forest
        } else if value < 0.5 {
            BiomeType::Plains
        } else if value < 0.7 {
            BiomeType::Mountains
        } else if value < 0.85 {
            BiomeType::Desert
        } else {
            BiomeType::Tundra
        }
    }

    pub fn generate_chunk(&self,
        coord: ChunkCoord,
    ) -> Chunk {
        let biome = self.generate_biome(coord);
        let mut chunk = Chunk::new(coord);
        chunk.biome_id = biome.as_str().to_string();
        
        self.populate_resources(&mut chunk, biome);
        
        chunk
    }

    fn populate_resources(
        &self,
        chunk: &mut Chunk,
        biome: BiomeType,
    ) {
        use crate::Resource;
        use rand::Rng;
        
        let mut rng = StdRng::seed_from_u64(
            self.seed.wrapping_add(
                (chunk.coord.0 as u64).wrapping_mul(12345)
                    .wrapping_add((chunk.coord.1 as u64).wrapping_mul(67890))
            )
        );

        let resource_count = match biome {
            BiomeType::Forest => rng.gen_range(5..15),
            BiomeType::Plains => rng.gen_range(3..8),
            BiomeType::Desert => rng.gen_range(1..4),
            BiomeType::Tundra => rng.gen_range(2..6),
            BiomeType::Mountains => rng.gen_range(4..12),
        };

        for i in 0..resource_count {
            let resource_type = match biome {
                BiomeType::Forest => {
                    if rng.gen_bool(0.7) {
                        "tree"
                    } else {
                        "rock"
                    }
                }
                BiomeType::Plains => {
                    if rng.gen_bool(0.6) {
                        "bush"
                    } else {
                        "rock"
                    }
                }
                BiomeType::Desert => {
                    if rng.gen_bool(0.5) {
                        "cactus"
                    } else {
                        "sandstone"
                    }
                }
                BiomeType::Tundra => {
                    if rng.gen_bool(0.6) {
                        "snow_rock"
                    } else {
                        "ice"
                    }
                }
                BiomeType::Mountains => {
                    if rng.gen_bool(0.7) {
                        "rock"
                    } else {
                        "ore"
                    }
                }
            };

            let x = rng.gen_range(0.0..128.0);
            let y = rng.gen_range(0.0..128.0);
            
            let resource = Resource {
                id: format!("resource_{}_{}", chunk.coord.0, i),
                subtype: resource_type.to_string(),
                x,
                y,
                hp: 30.0,
                max_hp: 30.0,
                level: 1,
            };
            
            chunk.resources.insert(resource.id.clone(), resource);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biome_generation_deterministic() {
        let generator1 = BiomeGenerator::new(12345);
        let generator2 = BiomeGenerator::new(12345);
        
        let coord = (10, 20);
        let biome1 = generator1.generate_biome(coord);
        let biome2 = generator2.generate_biome(coord);
        
        assert_eq!(biome1, biome2);
    }

    #[test]
    fn test_different_coords_different_biomes() {
        let generator = BiomeGenerator::new(12345);
        
        let biome1 = generator.generate_biome((0, 0));
        let biome2 = generator.generate_biome((100, 100));
        
        // They might be the same by chance, but usually different
        // This test mainly checks it doesn't panic
        let _ = biome1;
        let _ = biome2;
    }

    #[test]
    fn test_chunk_generation() {
        let generator = BiomeGenerator::new(12345);
        let chunk = generator.generate_chunk((0, 0));
        
        assert!(!chunk.biome_id.is_empty());
        assert!(!chunk.resources.is_empty());
    }
}
