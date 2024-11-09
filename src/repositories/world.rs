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

    /**
    Loads chunks around the given center

    # Arguments

    `center` - center of the chunk area
    `radius` - radius in number of chunks to be loaded
    */
    pub fn load_chunks(&self, center: &Coords, radius: u32) -> Vec<Vec<Chunk>> {
        let cs = CHUNK_SIZE as i32;

        let r = (radius as i32) * cs; // Radius in tiles

        // offsets in chunks quantity
        let left = (center.x() - r) / cs;
        let right = (center.x() + r) / cs;
        let bottom = (center.y() - r) / cs;
        let top = (center.y() + r) / cs;

        let mut chunks = Vec::new();

        for x in left..right {
            let mut column = Vec::new();

            for y in bottom..top {
                let is_odd = y % 2 == 1;
                let kind = if is_odd {
                    TileKind::Grass
                } else {
                    TileKind::Water
                };

                let chunk = Chunk::new(kind, Coords::new(x * cs, y * cs));
                column.push(chunk);
            }

            chunks.push(column);
        }

        return chunks;
    }
}
