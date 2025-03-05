use std::collections::HashMap;

use crate::world::tiles::TileKind;

enum TileSlot {
    Specific(TileKind),
    Any,
}

impl std::fmt::Display for TileSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TileSlot::Specific(kind) => kind.fmt(f),
            TileSlot::Any => write!(f, "slot"),
        }
    }
}

pub struct TilesetDict {
    dict: HashMap<String, TextureLayers>,
}

impl TilesetDict {
    pub fn new() -> Self {
        let mut h = HashMap::new();

        TilesetDict::setup_dirt_wildcards(&mut h);
        TilesetDict::setup_grass_wildcards(&mut h);

        return Self { dict: h };
    }

    fn setup_dirt_wildcards(h: &mut HashMap<String, TextureLayers>) {
        h.insert(
            TilesetDict::key_from([
                TileSlot::Specific(TileKind::Dirt),
                TileSlot::Any,
                TileSlot::Any,
                TileSlot::Specific(TileKind::Dirt),
            ]),
            TextureLayers::new().add(0, [1, 2]).clone(),
        );

        h.insert(
            TilesetDict::key_from([
                TileSlot::Specific(TileKind::Dirt),
                TileSlot::Specific(TileKind::Dirt),
                TileSlot::Specific(TileKind::Dirt),
                TileSlot::Any,
            ]),
            TextureLayers::new().add(0, [3, 1]).clone(),
        );

        h.insert(
            TilesetDict::key_from([
                TileSlot::Specific(TileKind::Dirt),
                TileSlot::Specific(TileKind::Dirt),
                TileSlot::Specific(TileKind::Dirt),
                TileSlot::Specific(TileKind::Dirt),
            ]),
            TextureLayers::new().add(0, [4, 1]).clone(),
        );

        h.insert(
            TilesetDict::key_from([
                TileSlot::Specific(TileKind::Dirt),
                TileSlot::Specific(TileKind::Dirt),
                TileSlot::Any,
                TileSlot::Any,
            ]),
            TextureLayers::new().add(0, [5, 0]).clone(),
        );

        h.insert(
            TilesetDict::key_from([
                TileSlot::Specific(TileKind::Dirt),
                TileSlot::Any,
                TileSlot::Any,
                TileSlot::Any,
            ]),
            TextureLayers::new().add(0, [0, 2]).clone(),
        );

        h.insert(
            TilesetDict::key_from([
                TileSlot::Any,
                TileSlot::Any,
                TileSlot::Specific(TileKind::Dirt),
                TileSlot::Specific(TileKind::Dirt),
            ]),
            TextureLayers::new().add(0, [3, 2]).clone(),
        );

        h.insert(
            TilesetDict::key_from([
                TileSlot::Any,
                TileSlot::Specific(TileKind::Dirt),
                TileSlot::Any,
                TileSlot::Any,
            ]),
            TextureLayers::new().add(0, [7, 2]).clone(),
        );

        h.insert(
            TilesetDict::key_from([
                TileSlot::Any,
                TileSlot::Any,
                TileSlot::Any,
                TileSlot::Specific(TileKind::Dirt),
            ]),
            TextureLayers::new().add(0, [9, 2]).clone(),
        );

        h.insert(
            TilesetDict::key_from([
                TileSlot::Any,
                TileSlot::Any,
                TileSlot::Specific(TileKind::Dirt),
                TileSlot::Any,
            ]),
            TextureLayers::new().add(0, [4, 0]).clone(),
        );

        h.insert(
            TilesetDict::key_from([
                TileSlot::Any,
                TileSlot::Specific(TileKind::Dirt),
                TileSlot::Any,
                TileSlot::Specific(TileKind::Dirt),
            ]),
            TextureLayers::new().add(0, [2, 1]).clone(),
        );

        h.insert(
            TilesetDict::key_from([
                TileSlot::Any,
                TileSlot::Specific(TileKind::Dirt),
                TileSlot::Specific(TileKind::Dirt),
                TileSlot::Any,
            ]),
            TextureLayers::new().add(0, [7, 0]).clone(),
        );

        h.insert(
            TilesetDict::key_from([
                TileSlot::Specific(TileKind::Dirt),
                TileSlot::Any,
                TileSlot::Specific(TileKind::Dirt),
                TileSlot::Any,
            ]),
            TextureLayers::new().add(0, [2, 8]).clone(),
        );

        h.insert(
            TilesetDict::key_from([
                TileSlot::Any,
                TileSlot::Specific(TileKind::Dirt),
                TileSlot::Specific(TileKind::Dirt),
                TileSlot::Specific(TileKind::Dirt),
            ]),
            TextureLayers::new().add(0, [6, 0]).clone(),
        );

        h.insert(
            TilesetDict::key_from([
                TileSlot::Specific(TileKind::Dirt),
                TileSlot::Any,
                TileSlot::Specific(TileKind::Dirt),
                TileSlot::Specific(TileKind::Dirt),
            ]),
            TextureLayers::new().add(0, [5, 1]).clone(),
        );

        h.insert(
            TilesetDict::key_from([
                TileSlot::Specific(TileKind::Dirt),
                TileSlot::Specific(TileKind::Dirt),
                TileSlot::Any,
                TileSlot::Specific(TileKind::Dirt),
            ]),
            TextureLayers::new().add(0, [2, 2]).clone(),
        );
    }

    fn setup_grass_wildcards(h: &mut HashMap<String, TextureLayers>) {
        h.insert(
            TilesetDict::key_from([
                TileSlot::Any,
                TileSlot::Specific(TileKind::Grass),
                TileSlot::Specific(TileKind::Grass),
                TileSlot::Any,
            ]),
            TextureLayers::new().add(0, [1, 2]).add(1, [3, 0]).clone(),
        );

        h.insert(
            TilesetDict::key_from([
                TileSlot::Any,
                TileSlot::Any,
                TileSlot::Any,
                TileSlot::Specific(TileKind::Grass),
            ]),
            TextureLayers::new().add(1, [6, 2]).clone(),
        );

        h.insert(
            TilesetDict::key_from([
                TileSlot::Specific(TileKind::Grass),
                TileSlot::Specific(TileKind::Grass),
                TileSlot::Specific(TileKind::Grass),
                TileSlot::Specific(TileKind::Grass),
            ]),
            TextureLayers::new().add(0, [0, 1]).clone(),
        );

        h.insert(
            TilesetDict::key_from([
                TileSlot::Any,
                TileSlot::Any,
                TileSlot::Specific(TileKind::Grass),
                TileSlot::Specific(TileKind::Grass),
            ]),
            TextureLayers::new().add(1, [9, 1]).clone(),
        );

        h.insert(
            TilesetDict::key_from([
                TileSlot::Any,
                TileSlot::Specific(TileKind::Grass),
                TileSlot::Specific(TileKind::Grass),
                TileSlot::Specific(TileKind::Grass),
            ]),
            TextureLayers::new().add(1, [2, 0]).clone(),
        );

        h.insert(
            TilesetDict::key_from([
                TileSlot::Specific(TileKind::Grass),
                TileSlot::Specific(TileKind::Grass),
                TileSlot::Any,
                TileSlot::Any,
            ]),
            TextureLayers::new().add(1, [1, 0]).clone(),
        );

        h.insert(
            TilesetDict::key_from([
                TileSlot::Specific(TileKind::Grass),
                TileSlot::Any,
                TileSlot::Specific(TileKind::Grass),
                TileSlot::Specific(TileKind::Grass),
            ]),
            TextureLayers::new().add(1, [1, 1]).clone(),
        );

        h.insert(
            TilesetDict::key_from([
                TileSlot::Specific(TileKind::Grass),
                TileSlot::Specific(TileKind::Grass),
                TileSlot::Any,
                TileSlot::Specific(TileKind::Grass),
            ]),
            TextureLayers::new().add(1, [8, 1]).clone(),
        );

        h.insert(
            TilesetDict::key_from([
                TileSlot::Specific(TileKind::Grass),
                TileSlot::Specific(TileKind::Grass),
                TileSlot::Specific(TileKind::Grass),
                TileSlot::Any,
            ]),
            TextureLayers::new().add(1, [9, 0]).clone(),
        );

        h.insert(
            TilesetDict::key_from([
                TileSlot::Specific(TileKind::Grass),
                TileSlot::Any,
                TileSlot::Specific(TileKind::Grass),
                TileSlot::Any,
            ]),
            TextureLayers::new().add(1, [5, 2]).clone(),
        );

        h.insert(
            TilesetDict::key_from([
                TileSlot::Any,
                TileSlot::Specific(TileKind::Grass),
                TileSlot::Any,
                TileSlot::Specific(TileKind::Grass),
            ]),
            TextureLayers::new().add(1, [8, 0]).clone(),
        );

        h.insert(
            TilesetDict::key_from([
                TileSlot::Specific(TileKind::Grass),
                TileSlot::Any,
                TileSlot::Any,
                TileSlot::Specific(TileKind::Grass),
            ]),
            TextureLayers::new().add(1, [7, 1]).clone(),
        );

        h.insert(
            TilesetDict::key_from([
                TileSlot::Specific(TileKind::Grass),
                TileSlot::Any,
                TileSlot::Any,
                TileSlot::Any,
            ]),
            TextureLayers::new().add(1, [6, 1]).clone(),
        );

        h.insert(
            TilesetDict::key_from([
                TileSlot::Any,
                TileSlot::Specific(TileKind::Grass),
                TileSlot::Any,
                TileSlot::Any,
            ]),
            TextureLayers::new().add(1, [4, 2]).clone(),
        );

        h.insert(
            TilesetDict::key_from([
                TileSlot::Any,
                TileSlot::Any,
                TileSlot::Specific(TileKind::Grass),
                TileSlot::Any,
            ]),
            TextureLayers::new().add(1, [0, 0]).clone(),
        );
    }

    fn key_from(tiles: [TileSlot; 4]) -> String {
        let arr = [
            tiles[0].to_string(),
            tiles[1].to_string(),
            tiles[2].to_string(),
            tiles[3].to_string(),
        ];

        return arr.join("_");
    }

    pub fn get(&self, tiles: [TileKind; 4]) -> Option<Vec<Vec<[u32; 2]>>> {
        let key = TilesetDict::key_from([
            TileSlot::Specific(tiles[0]),
            TileSlot::Specific(tiles[1]),
            TileSlot::Specific(tiles[2]),
            TileSlot::Specific(tiles[3]),
        ]);

        let exact = self.dict.get(&key);

        if exact.is_some() {
            return exact.and_then(|layer| Some(layer.to_vec()));
        }

        let pattern = self.get_from_pattern(tiles);
        if pattern.is_some() {
            return pattern.and_then(|layer| Some(layer.to_vec()));
        }

        return None;
    }

    fn get_from_pattern(&self, tiles: [TileKind; 4]) -> Option<TextureLayers> {
        let mut grouped: HashMap<TileKind, HashMap<usize, bool>> = HashMap::new();

        for (idx, tile) in tiles.iter().enumerate() {
            if !grouped.contains_key(tile) {
                grouped.insert(tile.clone(), HashMap::new());
            }

            grouped.get_mut(tile).unwrap().insert(idx, true);
        }

        let parts: Vec<TextureLayers> = grouped
            .values()
            .map(|v| {
                let slots: [TileSlot; 4] = std::array::from_fn(|i| {
                    if v.get(&i).is_some() {
                        return TileSlot::Specific(tiles[i as usize].clone());
                    }

                    return TileSlot::Any;
                });

                let key = TilesetDict::key_from(slots);
                return self
                    .dict
                    .get(&key)
                    .expect(&format!(
                        "Texture not found for layer pattern: {}, tiles were {:?}",
                        key.to_string(),
                        tiles
                    ))
                    .clone();
            })
            .collect();

        let t = TextureLayers::mix(parts);
        return Some(t);
    }
}

#[derive(Debug, Clone)]
struct TextureLayers {
    layers: Vec<Option<Vec<[u32; 2]>>>,
}

impl TextureLayers {
    fn new() -> Self {
        return Self { layers: vec![] };
    }

    fn add(&mut self, layer: u32, texture: [u32; 2]) -> &mut Self {
        for i in 0..=(layer) {
            if i >= self.layers.len() as u32 {
                self.layers.push(None);
            }

            if i == layer {
                self.layers[i as usize] = Some(vec![texture]);
            }
        }

        return self;
    }

    fn to_vec(&self) -> Vec<Vec<[u32; 2]>> {
        let mut vec = Vec::with_capacity(self.layers.len());

        for layer in self.layers.iter() {
            if let Some(layer) = layer {
                vec.push(layer.clone());
            }
        }

        return vec;
    }

    fn mix(layers: Vec<Self>) -> Self {
        let mut r = TextureLayers::new();

        for layer in layers {
            for (i, l) in layer.layers.iter().enumerate() {
                let Some(layer) = l else {
                    continue;
                };

                for texture in layer.iter() {
                    r.add(i as u32, texture.clone());
                }
            }
        }

        return r;
    }
}
