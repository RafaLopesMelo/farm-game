use std::sync::Mutex;

use crate::world::{camera::Camera, world::World};

use super::{textures::TextureAtlas, tiles::TileRender};

pub struct WorldRender {
    tiles: Vec<TileRender>,
}

impl WorldRender {
    pub fn new(world: &World, atlas: Mutex<&mut TextureAtlas>, camera: &Camera) -> Self {
        let tiles = world
            .chunks_vec()
            .iter()
            .flat_map(|chunk| {
                return chunk.tiles().iter().flat_map(|row| {
                    return row.iter().map(|tile| {
                        let mut a = atlas.lock().unwrap();
                        let texture = a.cached_texture(tile.as_ref()).or_else(|| {
                            let texture = a.texture_for_tile(tile.as_ref(), &world);
                            return Some(texture);
                        });

                        return TileRender::new(tile.as_ref(), texture.unwrap(), camera);
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

// [0.2, 0.0], uv_max: [0.299, 0.165]
