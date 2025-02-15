use std::collections::HashMap;

use crate::world::tiles::TileKind;

pub struct TilesetDict {
    dict: HashMap<String, Vec<Vec<[u32; 2]>>>,
}

impl TilesetDict {
    pub fn new() -> Self {
        let mut h = HashMap::new();

        h.insert(
            TilesetDict::key_from([
                TileKind::Grass,
                TileKind::Grass,
                TileKind::Grass,
                TileKind::Grass,
            ]),
            vec![vec![[0, 1]]],
        );

        return Self { dict: h };
    }

    fn key_from(tiles: [TileKind; 4]) -> String {
        let arr = [
            tiles[0].to_string(),
            tiles[1].to_string(),
            tiles[2].to_string(),
            tiles[3].to_string(),
        ];

        return arr.join("_");
    }

    pub fn get(&self, tiles: [TileKind; 4]) -> Option<Vec<Vec<[u32; 2]>>> {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_from() {
        let key = TilesetDict::key_from([
            TileKind::Grass,
            TileKind::Grass,
            TileKind::Grass,
            TileKind::Grass,
        ]);

        assert_eq!(key, "Grass_Grass_Grass_Grass");
    }

    #[test]
    fn test_grass_grass_grass_grass() {
        let dict = TilesetDict::new();
        let texture = dict.get([
            TileKind::Grass,
            TileKind::Grass,
            TileKind::Grass,
            TileKind::Grass,
        ]);

        assert_eq!(texture, Some([0, 1]));
    }
}
