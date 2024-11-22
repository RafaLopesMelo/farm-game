use std::collections::HashMap;

use super::{
    camera::Camera,
    chunks::{Chunk, CHUNK_SIZE},
    coords::Coords2D,
    generator::WorldGenerator,
    tiles::Tile,
};

pub struct World {
    chunks: HashMap<i32, HashMap<i32, Chunk>>,
    generator: WorldGenerator,
}

impl World {
    pub fn new() -> Self {
        return Self {
            chunks: HashMap::new(),
            generator: WorldGenerator::new(),
        };
    }

    pub fn load(&mut self, camera: &Camera) {
        let cam_coords = camera.coords();

        // @TODO: Handle first render
        let center = if cam_coords.x() == 0 && cam_coords.y() == 0 {
            Coords2D::new(0, 0)
        } else {
            self.chunk_at(camera.coords())
                .expect("camera coords not loaded")
                .coords()
                .clone()
        };

        let cs = CHUNK_SIZE as i32;
        let r = 10 * cs; // Radius in tiles

        // offsets in chunks quantity
        let left = (center.x() - r) / cs;
        let right = (center.x() + r) / cs;
        let bottom = (center.y() - r) / cs;
        let top = (center.y() + r) / cs;

        for x in left..right {
            for y in bottom..top {
                let already_loaded = self.chunk_at(Coords2D::new(x * cs, y * cs));
                if already_loaded.is_some() {
                    continue;
                }

                let coords = Coords2D::new(x * cs, y * cs);
                let chunk = self.generator.generate(coords);

                self.chunks
                    .entry(x)
                    .or_insert(HashMap::new())
                    .insert(y, chunk);
            }
        }
    }

    pub fn chunk_at(&self, coords: Coords2D) -> Option<&Chunk> {
        let chunks = self.chunks_vec();

        // TODO -> Optimize
        for row in chunks {
            for chunk in row {
                if chunk.contains(coords) {
                    return Some(chunk);
                }
            }
        }

        return None;
    }

    pub fn neighbors_of(&self, coords: Coords2D) -> [&dyn Tile; 4] {
        let tc = Coords2D::new(coords.x(), coords.y() + 1);
        let bc = Coords2D::new(coords.x(), coords.y() - 1);
        let rc = Coords2D::new(coords.x() + 1, coords.y());
        let lc = Coords2D::new(coords.x() - 1, coords.y());

        let t_chunk = self.chunk_at(tc);
        let b_chunk = self.chunk_at(bc);
        let r_chunk = self.chunk_at(rc);
        let l_chunk = self.chunk_at(lc);

        let t = t_chunk.and_then(|c| c.tile_at(coords));
        let b = b_chunk.and_then(|c| c.tile_at(coords));
        let r = r_chunk.and_then(|c| c.tile_at(coords));
        let l = l_chunk.and_then(|c| c.tile_at(coords));

        return [t.unwrap(), b.unwrap(), r.unwrap(), l.unwrap()];
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
