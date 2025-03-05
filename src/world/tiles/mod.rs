use std::{
    any::Any,
    fmt::{self, Display},
};

use super::coords::Coords3D;

pub mod dirt;
pub mod grass;
pub mod water;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TileKind {
    Dirt = 0,
    Grass = 1,
    Water = 2,
}

impl Display for TileKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait Tile: Any {
    fn kind(&self) -> TileKind;
    fn coords(&self) -> &Coords3D;
    fn walkable(&self) -> bool;
    fn as_any(&self) -> &dyn Any;
    fn is(&self, kind: TileKind) -> bool {
        return self.kind() as i32 == kind as i32;
    }
}
