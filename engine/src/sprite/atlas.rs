use std::{collections::HashMap, sync::Arc};

use crate::{math, sprite::SpriteId, texture::Texture};

pub struct Atlas {
    texture: Arc<Texture>,
    tile_size: glam::UVec2,
    regions: HashMap<SpriteId, AtlasRegion>,
}

struct AtlasRegion {
    id: SpriteId,
    uv: math::uv::UvRect,
}

pub struct AtlasRegionDescriptor {
    id: SpriteId,
    coords: glam::UVec2,
}

pub struct AtlasConfig {
    pub tile_size: glam::UVec2,
}

impl Atlas {
    pub fn new<I>(texture: Arc<Texture>, definitions: I, config: AtlasConfig) -> Self
    where
        I: IntoIterator<Item = AtlasRegionDescriptor>,
    {
        let dimensions = texture.dimensions();

        let mut regions: HashMap<SpriteId, AtlasRegion> = HashMap::new();
        for d in definitions {
            let uv = math::uv::UvRect::from_pixels(
                [
                    d.coords.x * config.tile_size.x,
                    d.coords.y * config.tile_size.y,
                ],
                [config.tile_size.x, config.tile_size.y],
                [dimensions.x, dimensions.y],
            );

            let result = regions.insert(d.id, AtlasRegion { id: d.id, uv });

            if result.is_some() {
                panic!("duplicated definition for sprite {}", d.id)
            }
        }

        return Self {
            texture,
            tile_size: config.tile_size,
            regions,
        };
    }

    pub fn region(&self, sprite_id: SpriteId) -> &AtlasRegion {
        return self.regions.get(&sprite_id).unwrap();
    }
}
