use super::{coords::Coords, generator::main::WorldGenerator, tiles::Tile};

/// The size of a chunk in tiles
pub const CHUNK_SIZE: u32 = 32;

pub struct Chunk {
    tiles: [[Tile; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
    coords: Coords, // left bottom
}

impl Chunk {
    pub fn new(coords: Coords) -> Self {
        let generator = WorldGenerator::new();

        let tiles = std::array::from_fn(|x| {
            return std::array::from_fn(|y| {
                return generator.generate(x as i32 + coords.x(), y as i32 + coords.y());
            });
        });

        return Self { tiles, coords };
    }

    pub fn tiles(&self) -> &[[Tile; CHUNK_SIZE as usize]; CHUNK_SIZE as usize] {
        return &self.tiles;
    }

    pub fn coords(&self) -> &Coords {
        return &self.coords;
    }

    pub fn contains(&self, coords: Coords) -> bool {
        let left = self.coords.x();
        let right = left + CHUNK_SIZE as i32;
        let top = self.coords.y();
        let bottom = top + CHUNK_SIZE as i32;

        let contains_x = coords.x() >= left && coords.x() < right;
        let contains_y = coords.y() >= top && coords.y() < bottom;

        return contains_x && contains_y;
    }
}
