mod grass;

use std::collections::HashMap;
use std::hash::Hash;

use crate::world::{
    coords::Coords3D,
    tiles::{Tile, TileKind},
    world::World,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Texture {
    uv_min: [f32; 2],
    uv_max: [f32; 2],
}

impl Texture {
    pub fn new(uv_min: [f32; 2], uv_max: [f32; 2]) -> Self {
        return Self { uv_min, uv_max };
    }
}

impl Eq for Texture {}

impl Hash for Texture {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.uv_min[0].to_bits().hash(state);
        self.uv_min[1].to_bits().hash(state);
        self.uv_max[0].to_bits().hash(state);
        self.uv_max[1].to_bits().hash(state);
    }
}

pub struct TextureAtlas {
    cache: HashMap<Coords3D, Texture>,
}

impl TextureAtlas {
    const ATLAS_ROWS: f32 = 6.0;
    const ATLAS_COLUMNS: f32 = 10.0;

    const TILE_TEXTURE: [[u32; 2]; 4] = [
        [0, 0],  // Grass
        [10, 1], // Dirt
        [1, 2],  // Hill
        [6, 4],  // Water
    ];

    pub fn new() -> Self {
        return Self {
            cache: HashMap::new(),
        };
    }

    pub fn cached_texture(&self, tile: &dyn Tile) -> Option<Texture> {
        return self.cache.get(&tile.coords()).cloned();
    }

    pub fn texture_for_tile(&mut self, tile: &dyn Tile, world: &World) -> Texture {
        let default = Self::texture_from_coords(Self::TILE_TEXTURE[0]);

        let texture = match tile.kind() {
            TileKind::Grass => grass::texture_for_tile(tile, world).unwrap_or(default),
            TileKind::Water => Self::texture_from_coords(Self::TILE_TEXTURE[3]),
            TileKind::Dirt => Self::texture_from_coords(Self::TILE_TEXTURE[1]),
        };

        self.cache.insert(tile.coords().clone(), texture);
        return texture;
    }

    pub fn texture_from_coords(coords: [u32; 2]) -> Texture {
        let c1 = coords[0] as f32;
        let c2 = coords[1] as f32;

        return Texture::new(
            [c1 / Self::ATLAS_COLUMNS, c2 / Self::ATLAS_ROWS],
            [
                (c1 + 0.99) / Self::ATLAS_COLUMNS,
                (c2 + 0.99) / Self::ATLAS_ROWS,
            ],
        );
    }
}
