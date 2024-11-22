use crate::world::tiles::Tile;

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
    const ATLAS_ROWS: f32 = 1.0;
    const ATLAS_COLUMNS: f32 = 3.0;

    const TILE_TEXTURE: [[f32; 2]; 3] = [
        [0.0, 0.0], // Dirt
        [1.0, 0.0], // Grass
        [2.0, 0.0], // Water
    ];

    pub fn new() -> Self {
        return Self {};
    }

    pub fn texture_for_tile(&self, tile: &dyn Tile) -> Texture {
        let kind = tile.kind() as usize;
        let texture = Self::TILE_TEXTURE[kind];

        return Texture::new(
            [
                texture[0] / Self::ATLAS_COLUMNS,
                texture[1] / Self::ATLAS_ROWS,
            ],
            [
                (texture[0] + 1.0) / Self::ATLAS_COLUMNS,
                (texture[1] + 1.0) / Self::ATLAS_ROWS,
            ],
        );
    }
}
