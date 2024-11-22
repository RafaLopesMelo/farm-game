use crate::world::{camera::Camera, world::World};

use super::tiles::TileRender;

pub struct WorldRender {
    tiles: Vec<TileRender>,
}

impl WorldRender {
    pub fn new(world: &World, camera: &Camera) -> Self {
        let tiles = world
            .chunks_vec()
            .iter()
            .flat_map(|row| {
                return row.iter().flat_map(|chunk| {
                    return chunk
                        .tiles()
                        .iter()
                        .flat_map(|row| {
                            return row.iter().map(|tile| {
                                return TileRender::new(tile.as_ref(), camera);
                            });
                        })
                        .collect::<Vec<TileRender>>();
                });
            })
            .collect::<Vec<TileRender>>();

        return WorldRender { tiles };
    }

    pub fn tiles(&self) -> Vec<TileRender> {
        return self.tiles.clone();
    }
}
