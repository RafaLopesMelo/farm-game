use std::collections::HashMap;

use crate::world::tiles::TileKind;

pub struct TextureDictionary {
    dict: HashMap<String, [u32; 2]>,
}

impl TextureDictionary {
    pub fn new() -> Self {
        let mut h = HashMap::new();

        h.insert(
            TextureDictionary::key_from([
                TileKind::Grass,
                TileKind::Grass,
                TileKind::Grass,
                TileKind::Grass,
            ]),
            [0, 0],
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

    pub fn get(&self, tiles: [TileKind; 4]) -> Option<[u32; 2]> {
        let key = TextureDictionary::key_from(tiles);
        return self.dict.get(&key).copied();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_from() {
        let key = TextureDictionary::key_from([
            TileKind::Grass,
            TileKind::Grass,
            TileKind::Grass,
            TileKind::Grass,
        ]);

        assert_eq!(key, "Grass_Grass_Grass_Grass");
    }
}
