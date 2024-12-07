use crate::world::{camera::Camera, world::World};

use super::{texture::TextureAtlas, tiles::TileRender};

pub struct WorldRender {
    tiles: Vec<TileRender>,
}

impl WorldRender {
    pub fn new(world: &World, camera: &Camera) -> Self {
        let atlas = TextureAtlas::new();

        let tiles = world
            .chunks_vec()
            .iter()
            .flat_map(|chunk| {
                return chunk.tiles().iter().flat_map(|row| {
                    return row.iter().map(|tile| {
                        let neighbors = world.neighbors_of(&tile.coords().to_2d());
                        let texture = atlas.texture_for_tile(tile.as_ref(), neighbors);

                        return TileRender::new(tile.as_ref(), texture, camera);
                    });
                });
            })
            .collect::<Vec<TileRender>>();

        return WorldRender { tiles };
    }

    pub fn tiles(&self) -> Vec<TileRender> {
        return self.tiles.clone();
    }
}
