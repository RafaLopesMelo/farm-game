use crate::repositories::world::WorldRepository;

use super::{camera::Camera, chunks::Chunk, coords::Coords};

pub struct World {
    chunks: Vec<Vec<Chunk>>,
}

impl World {
    pub fn new() -> Self {
        return Self { chunks: Vec::new() };
    }

    pub fn load(&mut self, camera: &Camera) {
        let repository = WorldRepository::new();
        self.chunks = repository.load_chunks(&self.find_current_chunk_coords(camera), 1);
    }

    fn find_current_chunk_coords(&self, camera: &Camera) -> Coords {
        let chunks = self.chunks();

        for row in chunks {
            for chunk in row {
                if chunk.contains(camera.coords) {
                    return chunk.coords().clone();
                }
            }
        }

        return camera.coords().clone();
    }

    pub fn chunks(&self) -> &Vec<Vec<Chunk>> {
        return &self.chunks;
    }
}
