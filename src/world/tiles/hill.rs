use crate::world::coords::Coords3D;

use super::{Tile, TileDirection, TileKind};

pub struct HillTile {
    coords: Coords3D,
    dir: TileDirection,
}

impl Tile for HillTile {
    fn kind(&self) -> TileKind {
        return TileKind::Hill;
    }

    fn coords(&self) -> Coords3D {
        return self.coords;
    }

    fn walkable(&self) -> bool {
        return false;
    }
}

impl HillTile {
    pub fn new(coords: Coords3D, dir: TileDirection) -> Self {
        return Self { coords, dir };
    }
}
