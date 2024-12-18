use std::collections::HashMap;

use crate::world::{
    coords::Coords3D,
    tiles::{Tile, TileKind},
};

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

pub struct TextureAtlas {
    cache: HashMap<Coords3D, Texture>,
}

impl TextureAtlas {
    const ATLAS_ROWS: f32 = 6.0;
    const ATLAS_COLUMNS: f32 = 10.0;

    const TILE_TEXTURE: [[f32; 2]; 4] = [
        [0.0, 0.0],  // Grass
        [10.0, 1.0], // Dirt
        [1.0, 2.0],  // Hill
        [6.0, 4.0],  // Water
    ];

    pub fn new() -> Self {
        return Self {
            cache: HashMap::new(),
        };
    }

    pub fn cached_texture(&self, tile: &dyn Tile) -> Option<Texture> {
        return self.cache.get(&tile.coords()).cloned();
    }

    pub fn texture_for_tile(
        &mut self,
        tile: &dyn Tile,
        neighbors: Option<[&dyn Tile; 4]>,
    ) -> Texture {
        let texture = self._texture_for_tile(tile, neighbors);
        self.cache.insert(tile.coords().clone(), texture);
        return texture;
    }

    fn _texture_for_tile(&self, tile: &dyn Tile, neighbors: Option<[&dyn Tile; 4]>) -> Texture {
        if tile.is(TileKind::Water) {
            return Self::texture_from_coords(Self::TILE_TEXTURE[3]);
        }

        if tile.is(TileKind::Grass) {
            return self.handle_grass(tile, neighbors);
        }

        if tile.is(TileKind::Dirt) {
            return Self::texture_from_coords(Self::TILE_TEXTURE[1]);
        }

        panic!("Unknown tile kind");
    }

    fn handle_grass(&self, tile: &dyn Tile, neighbors: Option<[&dyn Tile; 4]>) -> Texture {
        if neighbors.is_none() {
            return Self::texture_from_coords([0.0, 0.0]);
        }

        let coords = tile.coords();

        let n = neighbors.unwrap();

        let top = n[0];
        let right = n[1];
        let bottom = n[2];
        let left = n[3];

        if coords.higher_than(bottom.coords()) {
            let coords_r = right.coords();
            let coords_l = left.coords();

            let as_high_r = coords_r.as_high_as(coords);
            let as_high_l = coords_l.as_high_as(coords);

            if as_high_r && as_high_l {
                return Self::texture_from_coords([0.0, 1.0]);
            }

            if as_high_l {
                println!("High left");
                return Self::texture_from_coords([1.0, 0.0]);
            }

            if as_high_r {
                println!("High right");
                return Self::texture_from_coords([9.0, 0.0]);
            }

            return Self::texture_from_coords([1.0, 1.0]);
        }

        return Self::texture_from_coords([0.0, 0.0]);
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
