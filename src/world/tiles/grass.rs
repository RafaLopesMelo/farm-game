use crate::world::coords::Coords3D;

use super::{Tile, TileKind};

pub struct GrassTile {
    coords: Coords3D,
}

impl Tile for GrassTile {
    fn kind(&self) -> TileKind {
        return TileKind::Grass;
    }

    fn coords(&self) -> &Coords3D {
        return &self.coords;
    }

    fn walkable(&self) -> bool {
        return true;
    }
}

impl GrassTile {
    pub fn new(coords: Coords3D) -> Self {
        return Self { coords };
    }
}
