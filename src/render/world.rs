use crate::world::{chunks::CHUNK_SIZE, world::World};

use super::{chunks::ChunkRender, tiles::TileRender};

pub struct WorldRender {
    chunks: Vec<ChunkRender>,
}

impl WorldRender {
    pub fn new(world: World) -> Self {
        let chunk_offset = CHUNK_SIZE * TileRender::size();

        let chunks = world
            .chunks()
            .iter()
            .enumerate()
            .map(|(x, row)| {
                return row.iter().enumerate().map(move |(y, chunk)| {
                    return ChunkRender::new(
                        chunk,
                        [x as u32 * chunk_offset, y as u32 * chunk_offset],
                    );
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
