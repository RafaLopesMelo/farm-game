#[derive(Copy, Clone)]
pub enum TileKind {
    Grass = 0,
    Water = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Tile {
    coords: [i32; 2],
    kind: TileKind,
}

impl Tile {
    pub fn new(kind: TileKind, coords: [i32; 2]) -> Self {
        return Self { kind, coords };
    }

    pub fn kind(&self) -> TileKind {
        return self.kind;
    }

    pub fn coords(&self) -> [i32; 2] {
        return self.coords;
    }
}
