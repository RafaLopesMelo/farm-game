use crate::world::coords::Coords2D;

use super::{Tile, TileKind};

pub struct DirtTile {
    coords: Coords2D,
    height: i32,
}

impl Tile for DirtTile {
    fn kind(&self) -> TileKind {
        return TileKind::Dirt;
    }

    fn coords(&self) -> Coords2D {
        return self.coords;
    }

    fn walkable(&self) -> bool {
        return true;
    }

    fn height(&self) -> i32 {
        return self.height;
    }
}

impl DirtTile {
    pub fn new(coords: Coords2D, height: i32) -> Self {
        return Self { coords, height };
    }
}
