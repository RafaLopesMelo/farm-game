use std::{collections::HashMap, sync::Arc};

use crate::{
    math::{self, units::Pixels},
    sprite::SpriteId,
    texture::Texture,
};

pub struct Atlas {
    texture: Arc<Texture>,
    tile_size: [Pixels; 2],
    regions: HashMap<SpriteId, AtlasRegion>,
}

struct AtlasRegion {
    id: SpriteId,
    uv: math::uv::UvRect,
}

pub struct AtlasRegionDescriptor {
    pub id: SpriteId,
    pub coords: glam::UVec2,
}

pub struct AtlasConfig {
    pub tile_size: [Pixels; 2],
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
                    (d.coords.x as f32) * config.tile_size[0],
                    (d.coords.y as f32) * config.tile_size[1],
                ],
                [config.tile_size[0], config.tile_size[1]],
                [dimensions[0], dimensions[1]],
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
