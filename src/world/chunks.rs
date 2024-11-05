use super::tiles::{Tile, TileKind};

/// The size of a chunk in tiles
pub const CHUNK_SIZE: u32 = 32;

pub struct Chunk {
    tiles: [[Tile; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
    coords: [i32; 2], // left bottom
}

impl Chunk {
    pub fn new(kind: TileKind, coords: [i32; 2]) -> Self {
        let tiles = std::array::from_fn(|x| {
            return std::array::from_fn(|y| {
                Tile::new(kind, [x as i32 + coords[0], y as i32 + coords[1]])
            });
        });

        return Self { tiles, coords };
    }

    pub fn tiles(&self) -> &[[Tile; CHUNK_SIZE as usize]; CHUNK_SIZE as usize] {
        return &self.tiles;
    }
}
