use crate::ecs::entity::Entity;

/// A sparse set is a data structure for storing components
/// Provides O(1) insertion, removal, lookup, and cache-friendly iteration
pub struct SparseSet<T> {
    /// Maps entity ID to index in dense array
    sparse: Vec<Option<usize>>,

    /// Packed array of entity IDs
    dense: Vec<Entity>,

    /// Packed array of components matching dense array
    data: Vec<T>,

    capacity: usize,
}

impl<T> SparseSet<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        let mut dense = Vec::new();
        dense.reserve(capacity);

        return Self {
            sparse: vec![None; capacity],
            dense,
            data: Vec::new(),
            capacity,
        };
    }

    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        if !self.contains(entity) {
            return None;
        }

        let idx = self.sparse[entity.to_idx()].unwrap();
        let last_idx = self.dense.len() - 1;

        self.dense.swap(idx, last_idx);
        self.data.swap(idx, last_idx);

        if idx != last_idx {
            let swapped = self.dense[idx].to_idx();
            self.sparse[swapped] = Some(idx);
        }

        self.sparse[entity.to_idx()] = None;
        self.dense.pop();

        return Some(self.data.pop().unwrap());
    }

    /// Checks if an entity has a component
    pub fn contains(&self, entity: Entity) -> bool {
        let v = entity.to_idx();
        if v >= self.capacity {
            return false;
        }

        let sparse = self.sparse[v];
        if sparse.is_none() {
            return false;
        }

        let i = sparse.unwrap();

        if i >= self.dense.len() {
            return false;
        }

        let dense = self.dense[i];
        return entity.equals(&dense);
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        if !self.contains(entity) {
            return None;
        }

        let idx = self.sparse[entity.to_idx()].unwrap();
        return Some(&self.data[idx]);
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        if !self.contains(entity) {
            return None;
        }

        let idx = self.sparse[entity.to_idx()].unwrap();
        return Some(&mut self.data[idx]);
    }
}
