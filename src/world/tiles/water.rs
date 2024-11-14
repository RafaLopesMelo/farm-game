use crate::world::coords::Coords;

use super::{Tile, TileKind};

pub struct WaterTile {
    coords: Coords,
}

impl Tile for WaterTile {
    fn kind(&self) -> TileKind {
        return TileKind::Water;
    }

    fn coords(&self) -> Coords {
        return self.coords;
    }

    fn walkable(&self) -> bool {
        return false;
    }
}

impl WaterTile {
    pub fn new(coords: Coords) -> Self {
        return Self { coords };
    }
}
