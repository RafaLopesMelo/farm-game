pub mod grass;
pub mod water;

use super::coords::Coords2D;

#[derive(Copy, Clone)]
pub enum TileKind {
    Grass = 0,
    Water = 1,
}

pub const TILE_MAX_HEIGHT: i32 = 255;

pub trait Tile {
    fn kind(&self) -> TileKind;
    fn coords(&self) -> Coords2D;
    fn walkable(&self) -> bool;
    fn height(&self) -> i32;
}
