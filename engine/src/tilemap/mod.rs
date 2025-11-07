use crate::{
    math::coords::{Coords2D, Coords3D},
    sprite::SpriteId,
};

struct TileKind {
    id: u16,
    name: String,
    sprite: SpriteId,
}

pub struct Tile {
    kind_id: u16,
    coords: Coords3D<i32>,
}

impl Tile {
    pub fn new(kind_id: u16, coords: Coords3D<i32>) -> Self {
        return Self { kind_id, coords };
    }

    pub fn coords(&self) -> Coords3D<i32> {
        return self.coords;
    }
}

const CHUNK_SIZE: usize = 64;

struct Chunk {
    /// The tiles contained in the chunk
    /// A flat array was chosen instead of a multi-dimensional array because of better CPU caching
    /// The tiles are ordered from left to right, bottom to top
    tiles: [Tile; CHUNK_SIZE * CHUNK_SIZE],

    /// Coordinates of the left-bottom tile of the chunk
    coords: Coords2D<i32>,
}

impl Chunk {
    /// Creates a new chunk
    ///
    /// # Arguments
    /// * `tiles` - chunk tiles ordered from left to right, bottom to top
    pub fn new(tiles: [Tile; CHUNK_SIZE * CHUNK_SIZE]) -> Self {
        let coords = tiles[0].coords().to_2d();
        return Self { tiles, coords };
    }
}
