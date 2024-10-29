use super::tiles::Tile;

/// The size of a chunk in tiles
pub const CHUNK_SIZE: u32 = 32;

struct Chunk {
    blocks: [[Tile; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
}
