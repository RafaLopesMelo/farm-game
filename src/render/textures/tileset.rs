use std::collections::HashMap;

use crate::world::tiles::TileKind;

pub struct TilesetDict {
    dict: HashMap<String, Vec<Vec<[u32; 2]>>>,
}

/*
@TODO: add slot concept to be used like this:

```
TilesetDict::key_from([
        TileKind::Grass,
        TileKind::Grass,
        Slot,
        Slot,
    ]),
    vec![vec![[0, 1]]],
);
```

Slot simbolizes any tile type

*/
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

        h.insert(
            TilesetDict::key_from([
                TileKind::Dirt,
                TileKind::Dirt,
                TileKind::Dirt,
                TileKind::Dirt,
            ]),
            vec![vec![[4, 1]]],
        );

        h.insert(
            TilesetDict::key_from([
                TileKind::Dirt,
                TileKind::Dirt,
                TileKind::Dirt,
                TileKind::Grass,
            ]),
            vec![vec![[3, 1]], vec![[6, 2]]],
        );

        h.insert(
            TilesetDict::key_from([
                TileKind::Dirt,
                TileKind::Dirt,
                TileKind::Grass,
                TileKind::Dirt,
            ]),
            vec![vec![[2, 2]], vec![[0, 0]]],
        );

        h.insert(
            TilesetDict::key_from([
                TileKind::Dirt,
                TileKind::Grass,
                TileKind::Dirt,
                TileKind::Dirt,
            ]),
            vec![vec![[5, 1]], vec![[4, 2]]],
        );

        h.insert(
            TilesetDict::key_from([
                TileKind::Grass,
                TileKind::Dirt,
                TileKind::Dirt,
                TileKind::Dirt,
            ]),
            vec![vec![[6, 0]], vec![[6, 1]]],
        );

        h.insert(
            TilesetDict::key_from([
                TileKind::Grass,
                TileKind::Dirt,
                TileKind::Dirt,
                TileKind::Grass,
            ]),
            vec![vec![[7, 0]], vec![[7, 1]]],
        );

        h.insert(
            TilesetDict::key_from([
                TileKind::Dirt,
                TileKind::Grass,
                TileKind::Dirt,
                TileKind::Grass,
            ]),
            vec![vec![[2, 8]], vec![[8, 0]]],
        );

        h.insert(
            TilesetDict::key_from([
                TileKind::Grass,
                TileKind::Dirt,
                TileKind::Grass,
                TileKind::Dirt,
            ]),
            vec![vec![[2, 1]], vec![[5, 2]]],
        );

        h.insert(
            TilesetDict::key_from([
                TileKind::Grass,
                TileKind::Grass,
                TileKind::Grass,
                TileKind::Dirt,
            ]),
            vec![vec![[9, 2]], vec![[9, 0]]],
        );

        h.insert(
            TilesetDict::key_from([
                TileKind::Grass,
                TileKind::Grass,
                TileKind::Dirt,
                TileKind::Grass,
            ]),
            vec![vec![[4, 0]], vec![[8, 1]]],
        );

        h.insert(
            TilesetDict::key_from([
                TileKind::Grass,
                TileKind::Dirt,
                TileKind::Grass,
                TileKind::Grass,
            ]),
            vec![vec![[7, 2]], vec![[1, 1]]],
        );

        h.insert(
            TilesetDict::key_from([
                TileKind::Dirt,
                TileKind::Grass,
                TileKind::Grass,
                TileKind::Grass,
            ]),
            vec![vec![[0, 2]], vec![[2, 0]]],
        );

        h.insert(
            TilesetDict::key_from([
                TileKind::Grass,
                TileKind::Grass,
                TileKind::Dirt,
                TileKind::Dirt,
            ]),
            vec![vec![[3, 2]], vec![[1, 0]]],
        );

        h.insert(
            TilesetDict::key_from([
                TileKind::Dirt,
                TileKind::Dirt,
                TileKind::Grass,
                TileKind::Grass,
            ]),
            vec![vec![[5, 0]], vec![[9, 1]]],
        );

        h.insert(
            TilesetDict::key_from([
                TileKind::Dirt,
                TileKind::Grass,
                TileKind::Grass,
                TileKind::Dirt,
            ]),
            vec![vec![[1, 2]], vec![[3, 0]]],
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
        let key = TilesetDict::key_from(tiles);
        return self.dict.get(&key).cloned();
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
