use crate::repositories::world::WorldRepository;

use super::{camera::Camera, chunks::Chunk};

pub struct World {
    chunks: Vec<Vec<Chunk>>,
}

impl World {
    pub fn new() -> Self {
        return Self { chunks: Vec::new() };
    }

    pub fn load(&mut self, camera: &Camera) {
        let repository = WorldRepository::new();
        self.chunks = repository.load_chunks(self.find_current_chunk(camera), 4);
    }

    fn find_current_chunk(&self, camera: &Camera) -> &Chunk {
        let chunks = self.chunks();

        for row in chunks {
            for chunk in row {
                if chunk.contains(camera.coords) {
                    return chunk;
                }
            }
        }

        panic!("could not find current chunk");
    }

    pub fn chunks(&self) -> &Vec<Vec<Chunk>> {
        return &self.chunks;
    }
}
