use crate::world::world::World;

use super::{chunks::ChunkRender, tiles::TileRender};

pub struct WorldRender {
    chunks: Vec<ChunkRender>,
}

impl WorldRender {
    pub fn new(world: World) -> Self {
        let chunks = world
            .chunks()
            .iter()
            .map(|row| {
                return row.iter().map(move |chunk| {
                    return ChunkRender::new(chunk);
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
