use std::any::Any;

use super::{Tile, TileKind};
use crate::world::coords::Coords3D;

pub struct WaterTile {
    coords: Coords3D,
}

impl Tile for WaterTile {
    fn kind(&self) -> TileKind {
        return TileKind::Water;
    }

    fn coords(&self) -> &Coords3D {
        return &self.coords;
    }

    fn walkable(&self) -> bool {
        return false;
    }

    fn as_any(&self) -> &dyn Any {
        return self;
    }
}

impl WaterTile {
    pub fn new(coords: Coords3D) -> Self {
        return Self { coords };
    }
}
