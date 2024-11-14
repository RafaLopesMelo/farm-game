use super::{coords::Coords, tiles::Tile};

/// The size of a chunk in tiles
pub const CHUNK_SIZE: u32 = 32;

pub struct Chunk {
    tiles: [[Box<dyn Tile>; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
    coords: Coords, // left bottom
}

impl Chunk {
    pub fn new(
        coords: Coords,
        tiles: [[Box<dyn Tile>; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
    ) -> Self {
        return Self { tiles, coords };
    }

    pub fn tiles(&self) -> &[[Box<dyn Tile>; CHUNK_SIZE as usize]; CHUNK_SIZE as usize] {
        return &self.tiles;
    }

    pub fn coords(&self) -> &Coords {
        return &self.coords;
    }

    pub fn tile_at(&self, coords: Coords) -> Option<&dyn Tile> {
        if !self.contains(coords) {
            return None;
        }

        let rel_x = coords.x() - self.coords.x();
        let rel_y = coords.y() - self.coords.y();

        return Some(self.tiles[rel_x as usize][rel_y as usize].as_ref());
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
