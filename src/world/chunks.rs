use super::tiles::{Tile, TileKind};

/// The size of a chunk in tiles
pub const CHUNK_SIZE: u32 = 32;

pub struct Chunk {
    tiles: [[Tile; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
}

impl Chunk {
    pub fn new() -> Self {
        let tiles = [[Tile::new(TileKind::Water); CHUNK_SIZE as usize]; CHUNK_SIZE as usize];
        return Self { tiles };
    }

    pub fn tiles(&self) -> &[[Tile; CHUNK_SIZE as usize]; CHUNK_SIZE as usize] {
        return &self.tiles;
    }
}
