use crate::world::tiles::Tile;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextureCoords {
    uv_min: [f32; 2],
    uv_max: [f32; 2],
}

impl TextureCoords {
    pub fn new(uv_min: [f32; 2], uv_max: [f32; 2]) -> Self {
        return Self { uv_min, uv_max };
    }

    pub fn uv_min(&self) -> [f32; 2] {
        return [self.uv_min[0], self.uv_min[1]];
    }

    pub fn uv_max(&self) -> [f32; 2] {
        return [self.uv_max[0], self.uv_max[1]];
    }
}

pub struct TextureAtlas {}

impl TextureAtlas {
    const ATLAS_ROWS: f32 = 1.0;
    const ATLAS_COLUMNS: f32 = 2.0;

    const TILE_TEXTURE: [[f32; 2]; 2] = [
        [0.0, 0.0], // Grass
        [1.0, 0.0], // Water
    ];

    pub fn new() -> Self {
        return Self {};
    }

    pub fn coords_for_tile(&self, tile: &dyn Tile) -> TextureCoords {
        let kind = tile.kind() as usize;
        let texture = Self::TILE_TEXTURE[kind];

        return TextureCoords::new(
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
