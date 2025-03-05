use crate::world::{
    coords::{Coords2D, Coords3D},
    noises::{
        cache::NoiseCache,
        fractal::{FractalNoise, FractalNoiseGenerationDescriptor},
    },
    tiles::{dirt::DirtTile, grass::GrassTile, water::WaterTile, Tile},
};

pub struct PlainsBiome {
    fractal: FractalNoise,
    cache: NoiseCache,
}

impl PlainsBiome {
    pub fn new() -> Self {
        return Self {
            fractal: FractalNoise::new(),
            cache: NoiseCache::new(),
        };
    }

    pub fn tile(&self, coords: Coords2D) -> Box<dyn Tile> {
        let desc = FractalNoiseGenerationDescriptor {
            octaves: 6,
            persistence: 0.4,
            lacunarity: 2.0,
            frequency: 0.005,
        };

        let noise = self.fractal.generate(coords, &desc, None);
        let height = self.height(coords);

        let c = Coords3D::new_lattice(coords.lattice_x(), coords.lattice_y(), height);

        if noise <= -0.4 {
            return Box::new(DirtTile::new(c)) as Box<dyn Tile>; // TODO: Should be water
        }

        if noise > 0.1 && noise <= 0.15 || noise > -0.15 && noise <= -0.1 {
            return Box::new(DirtTile::new(c)) as Box<dyn Tile>;
        }

        if noise <= 1.0 {
            return Box::new(GrassTile::new(c)) as Box<dyn Tile>;
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

        let noise = self.fractal.generate(coords, &desc, Some(&self.cache));

        if noise < 0.0 {
            return 0;
        }

        return 1;
    }
}
