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
}

impl<T> SparseSet<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        let mut dense = Vec::new();
        dense.reserve(capacity);

        return Self {
            sparse: vec![None; capacity],
            dense,
            data: Vec::new(),
        };
    }

    pub fn insert(&mut self, entity: Entity, component: T) {
        let id = entity.to_idx();

        if id >= self.capacity() {
            self.sparse.resize(id + 1, None);
        }

        let sparse = self.sparse[id];

        if sparse.is_none() {
            self.dense.push(entity);
            self.data.push(component);

            self.sparse[id] = Some(self.dense.len() - 1);
            return;
        }

        let idx = sparse.unwrap();
        self.dense[idx] = entity;
        self.data[idx] = component;
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

    fn capacity(&self) -> usize {
        return self.sparse.len();
    }

    /// Checks if an entity has a component
    pub fn contains(&self, entity: Entity) -> bool {
        let v = entity.to_idx();
        if v >= self.capacity() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_new() {
        let mut s = SparseSet::<String>::with_capacity(10);
        let e = Entity::new(1);

        let value = "hello";

        s.insert(e, "hello".to_string());
        let v = s.get(e);

        assert!(v.is_some());
        assert_eq!(v.unwrap(), value);
    }

    #[test]
    fn test_insert_existing() {
        let mut s = SparseSet::<String>::with_capacity(10);
        let e = Entity::new(1);

        let value = "world";

        s.insert(e, "hello".to_string());
        s.insert(e, value.to_string());

        let v = s.get(e);

        assert!(v.is_some());
        assert_eq!(v.unwrap(), value);
    }

    #[test]
    fn test_insert_out_capacity() {
        let mut s = SparseSet::<String>::with_capacity(1);
        let e1 = Entity::new(1);
        let e2 = Entity::new(2);

        s.insert(e1, "".to_string());
        s.insert(e2, "".to_string());

        assert!(s.get(e1).is_some());
        assert!(s.get(e2).is_some());
    }

    #[test]
    fn test_remove() {
        let mut s = SparseSet::<String>::with_capacity(10);
        let e = Entity::new(1);

        let value = "hello";

        s.insert(e, "hello".to_string());
        let v = s.remove(e);

        assert!(v.is_some());
        assert_eq!(v.unwrap(), value)
    }

    #[test]
    fn test_remove_missing() {
        let mut s = SparseSet::<String>::with_capacity(10);
        let e = Entity::new(1);

        let v = s.remove(e);

        assert!(v.is_none());
    }
}
