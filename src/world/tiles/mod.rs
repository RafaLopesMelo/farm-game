use super::coords::Coords3D;

pub mod dirt;
pub mod grass;
pub mod water;

#[derive(Copy, Clone)]
pub enum TileKind {
    Dirt = 0,
    Grass = 1,
    Water = 2,
}

pub trait Tile {
    fn kind(&self) -> TileKind;
    fn coords(&self) -> &Coords3D;
    fn walkable(&self) -> bool;
    fn is(&self, kind: TileKind) -> bool {
        return self.kind() as i32 == kind as i32;
    }
}

pub enum TileDirection {
    Up,
    Down,
    Left,
    Right,
}
