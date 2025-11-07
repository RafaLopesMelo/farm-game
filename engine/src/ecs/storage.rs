use std::any::Any;

use crate::ecs::{
    component::{Component, ComponentStore},
    entity::Entity,
};

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

    pub fn insert(&mut self, entity: Entity, component: T) -> Option<T> {
        let id = entity.idx();

        if id >= self.capacity() {
            self.sparse.resize(id + 1, None);
        }

        let sparse = self.sparse[id];

        if sparse.is_none() {
            self.dense.push(entity);
            self.data.push(component);

            self.sparse[id] = Some(self.dense.len() - 1);
            return None;
        }

        let idx = sparse.unwrap();

        self.dense[idx] = entity;
        let old = std::mem::replace(&mut self.data[idx], component);

        return Some(old);
    }

    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        if !self.has(entity) {
            return None;
        }

        let idx = self.sparse[entity.idx()].unwrap();
        let last_idx = self.dense.len() - 1;

        self.dense.swap(idx, last_idx);
        self.data.swap(idx, last_idx);

        if idx != last_idx {
            let swapped = self.dense[idx].idx();
            self.sparse[swapped] = Some(idx);
        }

        self.sparse[entity.idx()] = None;
        self.dense.pop();

        return Some(self.data.pop().unwrap());
    }

    fn capacity(&self) -> usize {
        return self.sparse.len();
    }

    /// Checks if an entity has a component
    pub fn has(&self, entity: Entity) -> bool {
        let v = entity.idx();
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
        return entity == dense;
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        if !self.has(entity) {
            return None;
        }

        let idx = self.sparse[entity.idx()].unwrap();
        return Some(&self.data[idx]);
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        if !self.has(entity) {
            return None;
        }

        let idx = self.sparse[entity.idx()].unwrap();
        return Some(&mut self.data[idx]);
    }

    pub fn iter(&self) -> impl Iterator<Item = (Entity, &T)> {
        return self
            .dense
            .iter()
            .zip(self.data.iter())
            .map(|(k, v)| (*k, v));
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Entity, &mut T)> {
        return self
            .dense
            .iter()
            .zip(self.data.iter_mut())
            .map(|(k, v)| (*k, v));
    }
}

impl<T: Component> ComponentStore for SparseSet<T> {
    fn remove(&mut self, entity: Entity) {
        self.remove(entity);
    }

    fn as_any_ref(&self) -> &dyn Any {
        return self;
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        return self;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_new() {
        let mut s = SparseSet::<String>::with_capacity(10);
        let e = Entity::new(1, 0);

        let value = "hello";

        let old = s.insert(e, "hello".to_string());
        let v = s.get(e);

        assert!(old.is_none());
        assert!(v.is_some());
        assert_eq!(v.unwrap(), value);
    }

    #[test]
    fn test_insert_existing() {
        let mut s = SparseSet::<String>::with_capacity(10);
        let e = Entity::new(1, 0);

        let v1 = "hello";
        let v2 = "world";

        s.insert(e, "hello".to_string());
        let old = s.insert(e, v2.to_string());

        let v = s.get(e);

        assert!(v.is_some());
        assert_eq!(v.unwrap(), v2);

        assert!(old.is_some());
        assert_eq!(old.unwrap(), v1);
    }

    #[test]
    fn test_insert_out_capacity() {
        let mut s = SparseSet::<String>::with_capacity(1);
        let e1 = Entity::new(1, 0);
        let e2 = Entity::new(2, 0);

        s.insert(e1, "".to_string());
        s.insert(e2, "".to_string());

        assert!(s.get(e1).is_some());
        assert!(s.get(e2).is_some());
    }

    #[test]
    fn test_remove() {
        let mut s = SparseSet::<String>::with_capacity(10);
        let e = Entity::new(1, 0);

        let value = "hello";

        s.insert(e, "hello".to_string());
        let v = s.remove(e);

        assert!(v.is_some());
        assert_eq!(v.unwrap(), value)
    }

    #[test]
    fn test_remove_missing() {
        let mut s = SparseSet::<String>::with_capacity(10);
        let e = Entity::new(1, 0);

        let v = s.remove(e);

        assert!(v.is_none());
    }

    #[test]
    fn test_iter() {
        let mut s = SparseSet::<String>::with_capacity(10);
        let e = Entity::new(1, 0);

        let value = "hello";
        s.insert(e, value.to_string());

        let mut iter = s.iter();
        let v = iter.next().unwrap();
        assert_eq!(v.0, e);
        assert_eq!(v.1, value);
        assert!(iter.next().is_none());
    }
}
