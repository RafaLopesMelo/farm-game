#[derive(Copy, Clone)]
pub enum TileKind {
    Grass = 0,
    Water = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Tile {
    coordinate: [u32; 2],
    kind: TileKind,
}

impl Tile {
    pub fn new(kind: TileKind, coordinate: [u32; 2]) -> Self {
        return Self { kind, coordinate };
    }

    pub fn kind(&self) -> TileKind {
        return self.kind;
    }
}
