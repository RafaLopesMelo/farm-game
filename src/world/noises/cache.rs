use std::collections::HashMap;

use crate::world::coords::Coords2D;

pub struct NoiseCache {
    cache: HashMap<i32, HashMap<i32, f32>>,
}

impl NoiseCache {
    pub fn new() -> Self {
        return Self {
            cache: HashMap::new(),
        };
    }

    pub fn get(&self, x: i32, y: i32) -> Option<f32> {
        let row = self.cache.get(&x);

        if row.is_none() {
            return None;
        }

        let col = row.unwrap().get(&y);

        if col.is_none() {
            return None;
        }

        return Some(col.unwrap().clone());
    }

    pub fn set(&mut self, coords: Coords2D, value: f32) {
        self.cache
            .entry(coords.lattice_x())
            .or_insert(HashMap::new())
            .insert(coords.lattice_y(), value);
    }
}
