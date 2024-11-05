use crate::world::{
    camera::Camera,
    chunks::{Chunk, CHUNK_SIZE},
};

use super::tiles::TileRender;

const CHUNK_TILE_AMOUNT: usize = (CHUNK_SIZE * CHUNK_SIZE) as usize;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ChunkRender {
    tiles: [TileRender; CHUNK_TILE_AMOUNT],
}

impl ChunkRender {
    pub fn new(chunk: &Chunk, camera: &Camera) -> Self {
        let tiles: [TileRender; CHUNK_TILE_AMOUNT] = chunk
            .tiles()
            .iter()
            .flat_map(|row| {
                return row.iter().map(|tile| {
                    return TileRender::new(tile, camera);
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
