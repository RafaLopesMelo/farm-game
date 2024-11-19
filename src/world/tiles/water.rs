use crate::world::coords::Coords2D;

use super::{Tile, TileKind};

pub struct WaterTile {
    coords: Coords2D,
    height: i32,
}

impl Tile for WaterTile {
    fn kind(&self) -> TileKind {
        return TileKind::Water;
    }

    fn coords(&self) -> Coords2D {
        return self.coords;
    }

    fn walkable(&self) -> bool {
        return false;
    }

    fn height(&self) -> i32 {
        return self.height;
    }
}

impl WaterTile {
    pub fn new(coords: Coords2D, height: i32) -> Self {
        return Self { coords, height };
    }
}
