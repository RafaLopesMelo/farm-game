use std::collections::HashMap;

use super::{
    camera::Camera,
    chunks::{Chunk, CHUNK_SIZE},
    coords::Coords2D,
    generator::WorldGenerator,
    tiles::Tile,
};

pub struct World {
    chunks: HashMap<Coords2D, Chunk>,
    generator: WorldGenerator,
    neighbors_cache: HashMap<Coords2D, [&'static dyn Tile; 4]>,
}

impl World {
    pub fn new() -> Self {
        return Self {
            chunks: HashMap::new(),
            generator: WorldGenerator::new(),
            neighbors_cache: HashMap::new(),
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
        let r = 5 * cs; // Radius in tiles

        // offsets in chunks quantity
        let left = (center.x().floor() as i32 - r) / cs;
        let right = (center.x().floor() as i32 + r) / cs;
        let bottom = (center.y().floor() as i32 - r) / cs;
        let top = (center.y().floor() as i32 + r) / cs;

        for x in left..right {
            for y in bottom..top {
                let already_loaded = self.chunk_at(Coords2D::new_lattice(x * cs, y * cs));
                if already_loaded.is_some() {
                    println!("Already loaded");
                }
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

    pub fn neighbors_of(&self, coords: &Coords2D) -> Option<[&dyn Tile; 4]> {
        return None;

        let x = coords.lattice_x();
        let y = coords.lattice_y();

        let tc = Coords2D::new_lattice(x, y + 1);
        let bc = Coords2D::new_lattice(x, y - 1);
        let rc = Coords2D::new_lattice(x + 1, y);
        let lc = Coords2D::new_lattice(x - 1, y);

        let t_chunk = self.chunk_at(tc);
        let b_chunk = self.chunk_at(bc);
        let r_chunk = self.chunk_at(rc);
        let l_chunk = self.chunk_at(lc);

        let t = t_chunk.and_then(|c| c.tile_at(coords));
        let b = b_chunk.and_then(|c| c.tile_at(coords));
        let r = r_chunk.and_then(|c| c.tile_at(coords));
        let l = l_chunk.and_then(|c| c.tile_at(coords));

        if t.is_none() || b.is_none() || r.is_none() || l.is_none() {
            return None;
        }

        return Some([t.unwrap(), r.unwrap(), b.unwrap(), l.unwrap()]);
    }

    pub fn chunks_vec(&self) -> Vec<&Chunk> {
        return self.chunks.values().collect::<Vec<&Chunk>>();
    }
}
