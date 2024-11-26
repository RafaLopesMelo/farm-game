use crate::world::{
    coords::{Coords2D, Coords3D},
    noises::{
        cache::NoiseCache,
        fractal::{FractalNoise, FractalNoiseGenerationDescriptor},
    },
    tiles::{
        dirt::DirtTile, grass::GrassTile, hill::HillTile, water::WaterTile, Tile, TileDirection,
    },
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
            return Box::new(WaterTile::new(c)) as Box<dyn Tile>;
        }

        let edge = self.edge(c);

        if edge.is_some() {
            return Box::new(HillTile::new(c, edge.unwrap())) as Box<dyn Tile>;
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

    fn edge(&self, coords: Coords3D) -> Option<TileDirection> {
        let dirs = [
            ([1, 0], TileDirection::Right), // right
            ([-1, 0], TileDirection::Left), // left
            ([0, 1], TileDirection::Up),    // top
            ([0, -1], TileDirection::Down), // down
            ([1, 1], TileDirection::Up),    // right top
            ([-1, 1], TileDirection::Up),   // left top
            ([1, -1], TileDirection::Up),   // right bottom
            ([-1, -1], TileDirection::Up),  // left bottom
        ];

        for dir in dirs {
            let x = coords.lattice_x() + dir.0[0];
            let y = coords.lattice_y() + dir.0[1];
            let n = self.height(Coords2D::new_lattice(x, y));

            if n < coords.lattice_z() {
                return Some(dir.1);
            }
        }

        return None;
    }
}
