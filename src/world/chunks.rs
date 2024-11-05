use super::{
    coords::Coords,
    tiles::{Tile, TileKind},
};

/// The size of a chunk in tiles
pub const CHUNK_SIZE: u32 = 32;

pub struct Chunk {
    tiles: [[Tile; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
    coords: Coords, // left bottom
}

impl Chunk {
    pub fn new(kind: TileKind, coords: Coords) -> Self {
        let tiles = std::array::from_fn(|x| {
            return std::array::from_fn(|y| {
                Tile::new(
                    kind,
                    Coords::new(x as i32 + coords.x(), y as i32 + coords.y()),
                )
            });
        });

        return Self { tiles, coords };
    }

    pub fn tiles(&self) -> &[[Tile; CHUNK_SIZE as usize]; CHUNK_SIZE as usize] {
        return &self.tiles;
    }
}
