use crate::world::{coords::Coords2D, noises::fractal::FractalNoise, tiles::Tile};

struct Plains {
    fractal: FractalNoise,
}

impl Plains {
    pub fn new() -> Self {
        return Self {
            fractal: FractalNoise::new(),
        };
    }

    pub fn tile(coords: Coords2D) -> Box<dyn Tile> {
        unimplemented!()
    }
}
