use std::collections::HashMap;

use crate::world::{
    coords::{Coords2D, Coords3D},
    tiles::Tile,
    world::TileLocator,
};

use super::{tileset::TilesetDict, Texture};

pub struct TextureAtlas {
    dict: TilesetDict,
    cache: HashMap<Coords3D, Vec<Vec<Texture>>>,
}

impl TextureAtlas {
    const ATLAS_ROWS: f32 = 3.0;
    const ATLAS_COLUMNS: f32 = 10.0;

    pub fn new() -> Self {
        return Self {
            dict: TilesetDict::new(),
            cache: HashMap::new(),
        };
    }

    pub fn textures_for_tile(
        &mut self,
        tile: &dyn Tile,
        world: &dyn TileLocator,
    ) -> Vec<Vec<Texture>> {
        let cached = self.cache.get(&tile.coords());

        if cached.is_some() {
            return cached.unwrap().clone();
        }

        let default = Self::texture_from_coords([0, 0]);

        let coords = tile.coords();
        let offset = Coords2D::new(coords.x() - 0.5, coords.y() - 0.5);

        let tr = world.tile_at(Coords2D::new(offset.x() + 0.5, offset.y() + 0.5));
        let tl = world.tile_at(Coords2D::new(offset.x() - 0.5, offset.y() + 0.5));
        let br = world.tile_at(Coords2D::new(offset.x() + 0.5, offset.y() - 0.5));
        let bl = world.tile_at(Coords2D::new(offset.x() - 0.5, offset.y() - 0.5));

        if tr.is_none() || tl.is_none() || br.is_none() || bl.is_none() {
            return vec![vec![default]];
        }

        let layers = self.dict.get([
            tr.unwrap().kind(),
            tl.unwrap().kind(),
            br.unwrap().kind(),
            bl.unwrap().kind(),
        ]);

        if layers.is_none() {
            return vec![vec![default]];
        }

        let textures: Vec<Vec<Texture>> = layers
            .unwrap()
            .iter()
            .map(|layer| {
                return layer
                    .iter()
                    .map(|texture| return TextureAtlas::texture_from_coords(*texture))
                    .collect();
            })
            .collect();

        self.cache.insert(tile.coords().clone(), textures.clone());
        return textures;
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
