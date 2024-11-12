pub mod perlin;

use crate::world::{
    chunks::Chunk,
    coords::Coords,
    tiles::{Tile, TileKind},
};

use perlin::PerlinNoise;

pub struct WorldGenerator {}

impl WorldGenerator {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn generate(&self, chunk_coords: Coords) -> Chunk {
        let n = PerlinNoise::new();

        let cx = chunk_coords.x();
        let cy = chunk_coords.y();

        let tiles = std::array::from_fn(|rel_x| {
            return std::array::from_fn(|rel_y| {
                let coords = Coords::new(rel_x as i32 + cx, rel_y as i32 + cy);

                let noise = n.generate(coords);

                if noise < 0.5 {
                    return Tile::new(TileKind::Grass, coords);
                }

                if noise <= 1.0 {
                    return Tile::new(TileKind::Water, coords);
                }

                println!("noise: {}x{} -> {}", coords.x(), coords.y(), noise);
                panic!("Invalid noise value");
            });
        });

        return Chunk::new(chunk_coords, tiles);
    }
}
