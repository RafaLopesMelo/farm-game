use crate::{
    math::coords::{Coords2D, Coords3D},
    sprite::SpriteId,
};

struct TileKind {
    id: u16,
    name: String,
    sprite: SpriteId,
}

#[derive(Clone, Copy)]
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
    tiles: [Tile; CHUNK_SIZE * CHUNK_SIZE],

    /// Coordinates of the left-bottom tile of the chunk
    coords: Coords2D<i32>,
}

impl Chunk {
    /// The tiles are ordered from left to right, bottom to top
    /// Creates a new chunk
    ///
    /// # Arguments
    /// * `tiles` - chunk tiles ordered from left to right, bottom to top
    pub fn new(tiles: [Tile; CHUNK_SIZE * CHUNK_SIZE]) -> Self {
        let coords = tiles[0].coords().to_2d();
        return Self { tiles, coords };
    }

    fn contains(&self, coords: Coords2D<i32>) -> bool {
        if self.coords.x() > coords.x() {
            return false;
        }

        if self.coords.y() > coords.y() {
            return false;
        }

        let diff = coords - self.coords;
        if diff.x() >= CHUNK_SIZE as i32 {
            return false;
        }

        if diff.y() >= CHUNK_SIZE as i32 {
            return false;
        }

        return true;
    }

    fn index_of(&self, coords: Coords2D<i32>) -> usize {
        if !self.contains(coords) {
            panic!(
                "Tried to get the index of tile {}x{} out of the chunk {}x{}",
                coords.x(),
                coords.y(),
                self.coords.x(),
                self.coords.y()
            );
        }

        let diff = coords - self.coords;
        let offset = (diff.y() as usize) * CHUNK_SIZE;
        let idx = offset + (diff.x() as usize);

        let tile = self
            .tiles
            .get(idx)
            .expect("Tried to access a tile in an out of bounds index");

        if tile.coords().to_2d() != coords {
            panic!(
                "Incorrect chunk tile resolution. Expected: {}x{}; Wanted: {}x{}",
                coords.x(),
                coords.y(),
                tile.coords().x(),
                tile.coords().y(),
            );
        }

        return idx;
    }

    pub fn tile_at(&self, coords: Coords2D<i32>) -> &Tile {
        let idx = self.index_of(coords);
        return &self.tiles[idx];
    }

    pub fn tile_at_mut(&mut self, coords: Coords2D<i32>) -> &mut Tile {
        let idx = self.index_of(coords);
        return &mut self.tiles[idx];
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use super::*;

    #[test]
    fn test_tile_at() {
        let tiles: [Tile; CHUNK_SIZE * CHUNK_SIZE] = std::array::from_fn(|i| {
            let x = (i % CHUNK_SIZE) as i32;
            let y = (i / CHUNK_SIZE) as i32;

            let coords = Coords3D::new(x, y, 0);
            return Tile::new(0, coords);
        });

        let chunk = Chunk::new(tiles);
        let tt = vec![
            Coords2D::new(0, 0),
            Coords2D::new(45, 0),
            Coords2D::new(45, 63),
            Coords2D::new(63, 45),
            Coords2D::new(63, 63),
            Coords2D::new(0, 45),
            Coords2D::new(30, 10),
        ];

        for t in tt {
            let tile = chunk.tile_at(t);
            assert!(tile.coords().to_2d() == t);
        }
    }

    #[test]
    fn test_tile_at_out_of_bounds() {
        let tiles: [Tile; CHUNK_SIZE * CHUNK_SIZE] = std::array::from_fn(|i| {
            let x = (i % CHUNK_SIZE) as i32;
            let y = (i / CHUNK_SIZE) as i32;

            let coords = Coords3D::new(x, y, 0);
            return Tile::new(0, coords);
        });

        let chunk = Chunk::new(tiles);
        let tt = vec![
            Coords2D::new(64, 0),
            Coords2D::new(0, 64),
            Coords2D::new(64, 64),
            Coords2D::new(-10, 10),
            Coords2D::new(10, -10),
        ];

        for t in tt {
            let r = panic::catch_unwind(|| {
                chunk.tile_at(t);
            });

            assert!(r.is_err());
        }
    }
}
