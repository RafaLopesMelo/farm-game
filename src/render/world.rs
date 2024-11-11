use crate::world::{camera::Camera, world::World};

use super::{chunks::ChunkRender, tiles::TileRender};

pub struct WorldRender {
    chunks: Vec<ChunkRender>,
}

impl WorldRender {
    pub fn new(world: &World, camera: &Camera) -> Self {
        let chunks = world
            .chunks_vec()
            .iter()
            .map(|row| {
                return row.iter().map(|chunk| {
                    return ChunkRender::new(chunk, camera);
                });
            })
            .flatten()
            .collect::<Vec<ChunkRender>>();

        return WorldRender { chunks };
    }

    pub fn tiles(&self) -> Vec<TileRender> {
        let tiles = self
            .chunks
            .iter()
            .flat_map(|chunk| chunk.tiles())
            .collect::<Vec<TileRender>>();

        return tiles;
    }
}
