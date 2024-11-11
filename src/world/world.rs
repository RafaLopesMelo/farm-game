use std::collections::HashMap;

use crate::repositories::world::WorldRepository;

use super::{
    camera::Camera,
    chunks::{Chunk, CHUNK_SIZE},
    coords::Coords,
};

pub struct World {
    chunks: HashMap<i32, HashMap<i32, Chunk>>,
}

impl World {
    pub fn new() -> Self {
        return Self {
            chunks: HashMap::new(),
        };
    }

    pub fn load(&mut self, camera: &Camera) {
        let repository = WorldRepository::new();

        let center = self.find_current_chunk_coords(camera);

        let cs = CHUNK_SIZE as i32;
        let r = 10 * cs; // Radius in tiles

        // offsets in chunks quantity
        let left = (center.x() - r) / cs;
        let right = (center.x() + r) / cs;
        let bottom = (center.y() - r) / cs;
        let top = (center.y() + r) / cs;

        for x in left..right {
            for y in bottom..top {
                let already_loaded = self.get_chunk(Coords::new(x, y));
                if already_loaded.is_some() {
                    continue;
                }

                let chunk = Chunk::new(Coords::new(x * cs, y * cs));
                self.chunks
                    .entry(x)
                    .or_insert(HashMap::new())
                    .insert(y, chunk);
            }
        }
    }

    pub fn get_chunk(&self, coords: Coords) -> Option<&Chunk> {
        return self
            .chunks
            .get(&coords.x())
            .and_then(|x| x.get(&coords.y()));
    }

    fn find_current_chunk_coords(&self, camera: &Camera) -> Coords {
        let chunks = self.chunks_vec();

        for row in chunks {
            for chunk in row {
                if chunk.contains(camera.coords) {
                    return chunk.coords().clone();
                }
            }
        }

        return camera.coords().clone();
    }

    pub fn chunks_vec(&self) -> Vec<Vec<&Chunk>> {
        return self
            .chunks
            .values()
            .map(|x| {
                return x.values().collect::<Vec<&Chunk>>();
            })
            .collect::<Vec<Vec<&Chunk>>>();
    }
}
