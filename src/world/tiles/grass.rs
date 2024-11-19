use crate::world::coords::Coords2D;

use super::{Tile, TileKind};

pub struct GrassTile {
    coords: Coords2D,
    height: i32,
}

impl Tile for GrassTile {
    fn kind(&self) -> TileKind {
        return TileKind::Grass;
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

impl GrassTile {
    pub fn new(coords: Coords2D, height: i32) -> Self {
        return Self { coords, height };
    }
}
