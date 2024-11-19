use crate::world::{
    chunks::Chunk,
    coords::Coords2D,
    tiles::{grass::GrassTile, water::WaterTile, Tile},
};

use super::noises::fractal::FractalNoise;

pub struct WorldGenerator {}

impl WorldGenerator {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn generate(&self, chunk_coords: Coords2D) -> Chunk {
        let n = FractalNoise::new();

        let cx = chunk_coords.x();
        let cy = chunk_coords.y();

        let tiles = std::array::from_fn(|rel_x| {
            return std::array::from_fn(|rel_y| {
                let coords = Coords2D::new(rel_x as i32 + cx, rel_y as i32 + cy);

                let type_noise = n.generate(coords, 3, 1.0, 2.0, 0.005);

                let height_noise = (n.generate(coords, 6, 1.0, 2.0, 0.0005) + 1.0) / 2.0; // [-1, 1] -> [0, 1]
                let height = (height_noise * 10.0).floor() as i32; // [0, 1] -> [0, 255]

                if type_noise <= -0.2 {
                    return Box::new(WaterTile::new(coords, height)) as Box<dyn Tile>;
                }

                if type_noise <= 1.0 {
                    return Box::new(GrassTile::new(coords, height)) as Box<dyn Tile>;
                }

                println!("noise: {}x{} -> {}", coords.x(), coords.y(), type_noise);
                panic!("Invalid noise value");
            });
        });

        return Chunk::new(chunk_coords, tiles);
    }
}
