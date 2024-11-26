use crate::world::tiles::{Tile, TileKind};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Texture {
    uv_min: [f32; 2],
    uv_max: [f32; 2],
}

impl Texture {
    pub fn new(uv_min: [f32; 2], uv_max: [f32; 2]) -> Self {
        return Self { uv_min, uv_max };
    }
}

pub struct TextureAtlas {}

impl TextureAtlas {
    const ATLAS_ROWS: f32 = 2.0;
    const ATLAS_COLUMNS: f32 = 3.0;

    const TILE_TEXTURE: [[f32; 2]; 4] = [
        [0.0, 0.0], // Dirt
        [1.0, 0.0], // Grass
        [1.0, 1.0], // Hill
        [2.0, 0.0], // Water
    ];

    pub fn new() -> Self {
        return Self {};
    }

    pub fn texture_for_tile(&self, tile: &dyn Tile) -> Texture {
        if tile.is(TileKind::Water) {
            return Self::texture_from_coords(Self::TILE_TEXTURE[3]);
        }

        if tile.is(TileKind::Grass) {
            return Self::texture_from_coords(Self::TILE_TEXTURE[1]);
        }

        if tile.is(TileKind::Dirt) {
            return Self::texture_from_coords(Self::TILE_TEXTURE[0]);
        }

        if tile.is(TileKind::Hill) {
            return Self::texture_from_coords(Self::TILE_TEXTURE[2]);
        }

        panic!("Unknown tile kind");
    }

    fn texture_from_coords(coords: [f32; 2]) -> Texture {
        return Texture::new(
            [
                coords[0] / Self::ATLAS_COLUMNS,
                coords[1] / Self::ATLAS_ROWS,
            ],
            [
                (coords[0] + 0.99) / Self::ATLAS_COLUMNS,
                (coords[1] + 0.99) / Self::ATLAS_ROWS,
            ],
        );
    }
}
