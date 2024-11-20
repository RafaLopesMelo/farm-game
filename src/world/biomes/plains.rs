use crate::world::{
    coords::Coords2D,
    noises::fractal::{FractalNoise, FractalNoiseGenerationDescriptor},
    tiles::{dirt::DirtTile, grass::GrassTile, water::WaterTile, Tile},
};

pub struct PlainsBiome {
    fractal: FractalNoise,
}

impl PlainsBiome {
    pub fn new() -> Self {
        return Self {
            fractal: FractalNoise::new(),
        };
    }

    pub fn tile(&self, coords: Coords2D) -> Box<dyn Tile> {
        let desc = FractalNoiseGenerationDescriptor {
            octaves: 6,
            persistence: 0.4,
            lacunarity: 2.0,
            frequency: 0.005,
        };

        let noise = self.fractal.generate(coords, &desc);
        let height = self.height(coords);

        if noise <= -0.4 {
            return Box::new(WaterTile::new(coords, height)) as Box<dyn Tile>;
        }

        if noise > 0.1 && noise <= 0.16 {
            return Box::new(DirtTile::new(coords, height)) as Box<dyn Tile>;
        }

        if noise <= 1.0 {
            return Box::new(GrassTile::new(coords, height)) as Box<dyn Tile>;
        }

        println!("noise: {}x{} -> {}", coords.x(), coords.y(), noise);
        panic!("Invalid noise value");
    }

    fn height(&self, coords: Coords2D) -> i32 {
        let desc = FractalNoiseGenerationDescriptor {
            octaves: 6,
            persistence: 1.0,
            lacunarity: 2.0,
            frequency: 0.0005,
        };

        let noise = (self.fractal.generate(coords, &desc) + 1.0) / 2.0; // [-1, 1] -> [0, 1]
        let height = (noise * 10.0).floor() as i32; // [0, 1] -> [0, 10]
        return height;
    }
}
