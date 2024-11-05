use crate::repositories::world::WorldRepository;

use super::chunks::Chunk;

pub struct World {
    chunks: Vec<Vec<Chunk>>,
}

impl World {
    pub fn new() -> Self {
        let repository = WorldRepository::new();
        let chunks = repository.load_chunks([0, 0], 2);

        return Self { chunks };
    }

    pub fn chunks(&self) -> &Vec<Vec<Chunk>> {
        return &self.chunks;
    }
}
