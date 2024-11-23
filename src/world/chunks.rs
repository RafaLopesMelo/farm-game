use super::{coords::Coords2D, tiles::Tile};

/// The size of a chunk in tiles
pub const CHUNK_SIZE: u32 = 32;

pub struct Chunk {
    tiles: [[Box<dyn Tile>; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
    coords: Coords2D, // left bottom
}

impl Chunk {
    pub fn new(
        coords: Coords2D,
        tiles: [[Box<dyn Tile>; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
    ) -> Self {
        return Self { tiles, coords };
    }

    pub fn tiles(&self) -> &[[Box<dyn Tile>; CHUNK_SIZE as usize]; CHUNK_SIZE as usize] {
        return &self.tiles;
    }

    pub fn coords(&self) -> &Coords2D {
        return &self.coords;
    }

    pub fn tile_at(&self, coords: Coords2D) -> Option<&dyn Tile> {
        if !self.contains(coords) {
            return None;
        }

        let rel_x = coords.lattice_x() - self.coords.lattice_x();
        let rel_y = coords.lattice_y() - self.coords.lattice_y();

        return Some(self.tiles[rel_x as usize][rel_y as usize].as_ref());
    }

    pub fn contains(&self, coords: Coords2D) -> bool {
        let x = coords.lattice_x();
        let y = coords.lattice_y();

        let left = self.coords.lattice_x();
        let right = left + CHUNK_SIZE as i32;
        let top = self.coords.lattice_y();
        let bottom = top + CHUNK_SIZE as i32;

        let contains_x = x >= left && x < right;
        let contains_y = y >= top && y < bottom;

        return contains_x && contains_y;
    }
}
