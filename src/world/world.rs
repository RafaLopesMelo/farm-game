use super::{
    chunks::{Chunk, CHUNK_SIZE},
    tiles::TileKind,
};

pub struct World {
    chunks: Vec<Vec<Chunk>>,
}

impl World {
    pub fn new() -> Self {
        let mut chunks: Vec<Vec<Chunk>> = vec![];

        for x in 0..6 {
            let mut chunk = vec![];
            for y in 0..2 {
                let coords = [x * CHUNK_SIZE, y * CHUNK_SIZE];
                let is_odd = y % 2 == 1;
                let kind = if is_odd {
                    TileKind::Grass
                } else {
                    TileKind::Water
                };
                chunk.push(Chunk::new(kind, coords));
            }
            chunks.push(chunk);
        }

        return Self { chunks };
    }

    pub fn chunks(&self) -> &Vec<Vec<Chunk>> {
        return &self.chunks;
    }
}
