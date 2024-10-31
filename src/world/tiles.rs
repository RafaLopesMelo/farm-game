#[derive(Copy, Clone)]
pub enum TileKind {
    Grass = 0,
    Water = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Tile {
    kind: TileKind,
}

impl Tile {
    pub fn new(kind: TileKind) -> Self {
        return Self { kind };
    }

    pub fn kind(&self) -> TileKind {
        return self.kind;
    }
}
