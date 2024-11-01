use super::tiles::{Tile, TileKind};

/// The size of a chunk in tiles
pub const CHUNK_SIZE: u32 = 32;

pub struct Chunk {
    tiles: [[Tile; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
}

impl Chunk {
    pub fn new(kind: TileKind) -> Self {
        let tiles = [[Tile::new(kind); CHUNK_SIZE as usize]; CHUNK_SIZE as usize];
        return Self { tiles };
    }

    pub fn tiles(&self) -> &[[Tile; CHUNK_SIZE as usize]; CHUNK_SIZE as usize] {
        return &self.tiles;
    }
}
