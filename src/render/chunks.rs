use crate::world::chunks::{Chunk, CHUNK_SIZE};

use super::tiles::TileRender;

const CHUNK_TILE_AMOUNT: usize = (CHUNK_SIZE * CHUNK_SIZE) as usize;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ChunkRender {
    tiles: [TileRender; CHUNK_TILE_AMOUNT],
}

impl ChunkRender {
    pub fn new(chunk: &Chunk, position: [u32; 2]) -> Self {
        let tile_size = TileRender::size();

        let tiles: [TileRender; CHUNK_TILE_AMOUNT] = chunk
            .tiles()
            .iter()
            .enumerate()
            .flat_map(|(x, row)| {
                return row.iter().enumerate().map(move |(y, tile)| {
                    return TileRender::new(
                        tile,
                        [
                            x as u32 * tile_size + position[0],
                            y as u32 * tile_size + position[1],
                        ],
                    );
                });
            })
            .collect::<Vec<TileRender>>()
            .try_into()
            .unwrap();

        return ChunkRender { tiles };
    }

    pub fn tiles(&self) -> [TileRender; CHUNK_TILE_AMOUNT] {
        return self.tiles;
    }
}
