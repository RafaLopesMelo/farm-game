use crate::world::{
    coords::Coords,
    tiles::{Tile, TileKind},
};

use super::perlin::PerlinNoise;

pub struct WorldGenerator {}

impl WorldGenerator {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn generate(&self, x: i32, y: i32) -> Tile {
        let coords = Coords::new(x, y);

        let n = PerlinNoise::new();
        let noise = n.generate(x, y);

        if noise < 0.5 {
            return Tile::new(TileKind::Grass, coords);
        }

        if noise <= 1.0 {
            return Tile::new(TileKind::Water, coords);
        }

        println!("noise: {}x{} -> {}", x, y, noise);
        panic!("Invalid noise value");
    }
}
