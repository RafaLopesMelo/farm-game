use crate::world::coords::Coords3D;

use super::{Tile, TileKind};

pub struct DirtTile {
    coords: Coords3D,
}

impl Tile for DirtTile {
    fn kind(&self) -> TileKind {
        return TileKind::Dirt;
    }

    fn coords(&self) -> &Coords3D {
        return &self.coords;
    }

    fn walkable(&self) -> bool {
        return true;
    }
}

impl DirtTile {
    pub fn new(coords: Coords3D) -> Self {
        return Self { coords };
    }
}
