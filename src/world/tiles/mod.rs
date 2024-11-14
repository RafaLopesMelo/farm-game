pub mod grass;
pub mod water;

use super::coords::Coords;

#[derive(Copy, Clone)]
pub enum TileKind {
    Grass = 0,
    Water = 1,
}

pub trait Tile {
    fn kind(&self) -> TileKind;
    fn coords(&self) -> Coords;
    fn walkable(&self) -> bool;
}
