use super::coords::Coords;

#[derive(Copy, Clone)]
pub enum TileKind {
    Grass = 0,
    Water = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Tile {
    coords: Coords,
    kind: TileKind,
}

impl Tile {
    pub fn new(kind: TileKind, coords: Coords) -> Self {
        return Self { kind, coords };
    }

    pub fn kind(&self) -> TileKind {
        return self.kind;
    }

    pub fn coords(&self) -> Coords {
        return self.coords;
    }
}
