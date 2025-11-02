use std::{collections::HashMap, sync::Arc};

use crate::texture::Texture;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpriteId(pub u16);

impl std::fmt::Display for SpriteId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Atlas {
    texture: Arc<Texture>,
    texture_size: glam::UVec2,
    tile_size: glam::UVec2,
    regions: HashMap<SpriteId, AtlasRegion>,
}

struct AtlasRegion {
    id: SpriteId,
    uv: glam::Vec2,
}

pub struct AtlasRegionDescriptor {
    id: SpriteId,
    coords: glam::UVec2,
}

pub struct AtlasConfig {
    pub texture_size: glam::UVec2,
    pub tile_size: glam::UVec2,
}

impl Atlas {
    pub fn new<I>(texture: Arc<Texture>, definitions: I, config: AtlasConfig) -> Self
    where
        I: IntoIterator<Item = AtlasRegionDescriptor>,
    {
        let uv_tile_width = config.tile_size.x / config.texture_size.x;
        let uv_tile_height = config.tile_size.y / config.texture_size.y;

        let mut regions: HashMap<SpriteId, AtlasRegion> = HashMap::new();
        for d in definitions {
            let result = regions.insert(
                d.id,
                AtlasRegion {
                    id: d.id,
                    uv: glam::Vec2::new(
                        (d.coords.x * uv_tile_width) as f32,
                        (d.coords.y * uv_tile_height) as f32,
                    ),
                },
            );

            if result.is_some() {
                panic!("duplicated definition for sprite {}", d.id)
            }
        }

        return Self {
            texture,
            texture_size: config.texture_size,
            tile_size: config.tile_size,
            regions,
        };
    }

    pub fn region(&self, sprite_id: SpriteId) -> &AtlasRegion {
        return self.regions.get(&sprite_id).unwrap();
    }
}
