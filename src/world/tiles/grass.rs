use crate::world::coords::Coords;

use super::{Tile, TileKind};

pub struct GrassTile {
    coords: Coords,
}

impl Tile for GrassTile {
    fn kind(&self) -> TileKind {
        return TileKind::Grass;
    }

    fn coords(&self) -> Coords {
        return self.coords;
    }

    fn walkable(&self) -> bool {
        return true;
    }
}

impl GrassTile {
    pub fn new(coords: Coords) -> Self {
        return Self { coords };
    }
}
