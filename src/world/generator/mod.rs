use crate::world::{chunks::Chunk, coords::Coords2D};

use super::biomes::plains::PlainsBiome;

pub struct WorldGenerator {}

impl WorldGenerator {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn generate(&self, chunk_coords: Coords2D) -> Chunk {
        let cx = chunk_coords.x();
        let cy = chunk_coords.y();

        let biome = PlainsBiome::new();

        let tiles = std::array::from_fn(|rel_x| {
            return std::array::from_fn(|rel_y| {
                let coords = Coords2D::new(rel_x as i32 + cx, rel_y as i32 + cy);
                return biome.tile(coords);
            });
        });

        return Chunk::new(chunk_coords, tiles);
    }
}
