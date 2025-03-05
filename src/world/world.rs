use std::collections::HashMap;

use super::{
    camera::Camera,
    chunks::{Chunk, CHUNK_SIZE},
    coords::Coords2D,
    generator::WorldGenerator,
    tiles::Tile,
};

pub trait TileLocator {
    fn tile_at(&self, coords: Coords2D) -> Option<&dyn Tile>;
}

pub struct World {
    chunks: HashMap<Coords2D, Chunk>,
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
        let center = if cam_coords.x() == 0.0 && cam_coords.y() == 0.0 {
            Coords2D::new(0.0, 0.0)
        } else {
            self.chunk_at(camera.coords())
                .expect("camera coords not loaded")
                .coords()
                .clone()
        };

        let cs = CHUNK_SIZE as i32;
        let r = 2 * cs; // Radius in tiles

        // offsets in chunks quantity
        let left = (center.x().floor() as i32 - r) / cs;
        let right = (center.x().floor() as i32 + r) / cs;
        let bottom = (center.y().floor() as i32 - r) / cs;
        let top = (center.y().floor() as i32 + r) / cs;

        for x in left..right {
            for y in bottom..top {
                let already_loaded = self.chunk_at(Coords2D::new_lattice(x * cs, y * cs));
                if already_loaded.is_some() {
                    continue;
                }

                let coords = Coords2D::new_lattice(x * cs, y * cs);

                let chunk = self.generator.generate(coords);

                self.chunks
                    .entry(Coords2D::new_lattice(x * cs, y * cs))
                    .or_insert(chunk);
            }
        }
    }

    pub fn chunk_at(&self, coords: Coords2D) -> Option<&Chunk> {
        let cx = (coords.x() / CHUNK_SIZE as f32).floor() as i32 * 32;
        let cy = (coords.y() / CHUNK_SIZE as f32).floor() as i32 * 32;

        return self.chunks.get(&Coords2D::new_lattice(cx, cy));
    }

    pub fn chunks_vec(&self) -> Vec<&Chunk> {
        return self.chunks.values().collect::<Vec<&Chunk>>();
    }
}

impl TileLocator for World {
    fn tile_at(&self, coords: Coords2D) -> Option<&dyn Tile> {
        let chunk = self.chunk_at(coords)?;
        return chunk.tile_at(&coords);
    }
}
