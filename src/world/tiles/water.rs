use crate::world::coords::Coords3D;

use super::{Tile, TileKind};

pub struct WaterTile {
    coords: Coords3D,
}

impl Tile for WaterTile {
    fn kind(&self) -> TileKind {
        return TileKind::Water;
    }

    fn coords(&self) -> Coords3D {
        return self.coords;
    }

    fn walkable(&self) -> bool {
        return false;
    }
}

impl WaterTile {
    pub fn new(coords: Coords3D) -> Self {
        return Self { coords };
    }
}
