use super::{chunks::Chunk, tiles::TileKind};

pub struct World {
    chunks: Vec<Vec<Chunk>>,
}

impl World {
    pub fn new() -> Self {
        let chunks = vec![
            vec![Chunk::new(TileKind::Grass), Chunk::new(TileKind::Water)],
            vec![Chunk::new(TileKind::Grass), Chunk::new(TileKind::Water)],
            vec![Chunk::new(TileKind::Grass), Chunk::new(TileKind::Water)],
            vec![Chunk::new(TileKind::Grass), Chunk::new(TileKind::Water)],
            vec![Chunk::new(TileKind::Grass), Chunk::new(TileKind::Water)],
            vec![Chunk::new(TileKind::Grass), Chunk::new(TileKind::Water)],
        ];

        return Self { chunks };
    }

    pub fn chunks(&self) -> &Vec<Vec<Chunk>> {
        return &self.chunks;
    }
}
