use crate::world::{
    chunks::{Chunk, CHUNK_SIZE},
    coords::Coords,
    tiles::TileKind,
};

pub struct WorldRepository {}

impl WorldRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub fn load_chunks(&self, center: [i32; 2], radius: u32) -> Vec<Vec<Chunk>> {
        let r = radius as i32;

        let left = center[0] - r;
        let right = center[0] + r;
        let top = center[1] - r;
        let bottom = center[1] + r;

        let mut chunks = Vec::new();

        let cs = CHUNK_SIZE as i32;
        let mut kind_idx = 0;
        for x in left..right {
            let mut column = Vec::new();

            for y in top..bottom {
                let is_odd = kind_idx % 2 == 1;
                let kind = if is_odd {
                    TileKind::Grass
                } else {
                    TileKind::Water
                };

                let chunk = Chunk::new(kind, Coords::new(x * cs, y * cs));
                column.push(chunk);
                kind_idx += 1;
            }

            chunks.push(column);
        }

        return chunks;
    }
}