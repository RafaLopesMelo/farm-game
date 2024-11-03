use super::tiles::{Tile, TileKind};

/// The size of a chunk in tiles
pub const CHUNK_SIZE: u32 = 32;

pub struct Chunk {
    tiles: [[Tile; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
    coordinate: [u32; 2], // left bottom
}

impl Chunk {
    pub fn new(kind: TileKind, coordinate: [u32; 2]) -> Self {
        let tiles = std::array::from_fn(|x| {
            return std::array::from_fn(|y| {
                Tile::new(kind, [x as u32 + coordinate[0], y as u32 + coordinate[1]])
            });
        });

        return Self { tiles, coordinate };
    }

    pub fn tiles(&self) -> &[[Tile; CHUNK_SIZE as usize]; CHUNK_SIZE as usize] {
        return &self.tiles;
    }
}
