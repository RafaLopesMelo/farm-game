use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::{
    ecs::{
        component::{Component, ComponentStore},
        entity::{Entity, EntityAllocator},
        storage::SparseSet,
    },
    utils,
};

pub struct World {
    entity_allocator: EntityAllocator,
    stores: HashMap<TypeId, Box<dyn ComponentStore>>,
}

impl World {
    pub fn new() -> Self {
        return Self {
            entity_allocator: EntityAllocator::new(),
            stores: HashMap::new(),
        };
    }

    pub fn spawn(&mut self) -> Entity {
        let e = self.entity_allocator.alloc();
        return e;
    }

    pub fn despawn(&mut self, entity: Entity) {
        self.entity_allocator.free(entity);

        for (_, store) in &mut self.stores {
            store.remove(entity);
        }
    }

    pub fn add_component<C: Component>(&mut self) {
        let t = TypeId::of::<C>();

        let exists = self.stores.get(&t);
        if exists.is_some() {
            return;
        }

        let b = Box::new(SparseSet::<C>::with_capacity(10));
        self.stores.insert(t, b);
    }

    fn get_downcasted_store_ref<C: Component>(&self) -> &SparseSet<C> {
        let s = self.stores.get(&TypeId::of::<C>()).unwrap_or_else(|| {
            panic!(
                "tried to insert on unregistered component: {}",
                std::any::type_name::<C>()
            )
        });

        return utils::as_any_ref(s)
            .downcast_ref::<SparseSet<C>>()
            .expect("internal error: component store type mismatch");
    }

    fn get_downcasted_store_mut<C: Component>(&mut self) -> &mut SparseSet<C> {
        let s = self.stores.get_mut(&TypeId::of::<C>()).unwrap_or_else(|| {
            panic!(
                "tried to insert on unregistered component: {}",
                std::any::type_name::<C>()
            )
        });

        return utils::as_any_mut(s)
            .downcast_mut::<SparseSet<C>>()
            .expect("internal error: component store type mismatch");
    }

    pub fn insert<C: Component>(&mut self, entity: Entity, component: C) -> Option<C> {
        let store = self.get_downcasted_store_mut::<C>();
        return store.insert(entity, component);
    }
    pub fn remove<C: Component>(&mut self, entity: Entity) -> Option<C> {
        let store = self.get_downcasted_store_mut::<C>();
        return store.remove(entity);
    }

    pub fn get<C: Component>(&self, entity: Entity) -> Option<&C> {
        let store = self.get_downcasted_store_ref::<C>();
        return store.get(entity);
    }

    pub fn get_mut<C: Component>(&mut self, entity: Entity) -> Option<&mut C> {
        let store = self.get_downcasted_store_mut::<C>();
        return store.get_mut(entity);
    }

    pub fn has<C: Component>(&self, entity: Entity) -> bool {
        let store = self.get_downcasted_store_ref::<C>();
        return store.has(entity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestComponent {
        pub value: String,
    }
    impl Component for TestComponent {}

    #[test]
    fn test_spawn() {
        let mut world = World::new();
        let e = world.spawn();
        world.despawn(e);
    }

    #[test]
    #[should_panic]
    fn test_insert_unregistered() {
        let mut world = World::new();
        let e = world.spawn();

        world.insert(
            e,
            TestComponent {
                value: "test".to_string(),
            },
        );
    }

    #[test]
    fn test_insert() {
        let mut world = World::new();
        world.add_component::<TestComponent>();

        let e = world.spawn();

        let value = "test";

        let old = world.insert(
            e,
            TestComponent {
                value: value.to_string(),
            },
        );
        assert!(old.is_none());

        // Ensure that `has` is working properly
        let has = world.has::<TestComponent>(e);
        assert!(has);

        // Ensure that `get` is working properly
        let val = world.get::<TestComponent>(e);
        assert_eq!(val.unwrap().value, value);

        // Ensure that `get_mut` is working properly
        let val = world.get_mut::<TestComponent>(e);
        assert_eq!(val.unwrap().value, value);

        // Ensure that the old value is returned if any
        let old = world.insert(
            e,
            TestComponent {
                value: value.to_string(),
            },
        );
        assert!(old.is_some());
        assert_eq!(old.unwrap().value, value);
    }

    #[test]
    fn test_remove() {
        let mut world = World::new();
        world.add_component::<TestComponent>();

        let e = world.spawn();

        // Ensure `None` return if nothing was removed
        let old = world.remove::<TestComponent>(e);
        assert!(old.is_none());

        let value = "test";
        world.insert(
            e,
            TestComponent {
                value: value.to_string(),
            },
        );

        // Ensure that removed component is returned after removal
        let old = world.remove::<TestComponent>(e);
        assert_eq!(old.unwrap().value, value);

        // Ensure that `has` is working properly
        let has = world.has::<TestComponent>(e);
        assert!(!has);

        // Ensure that `get` is working properly
        let val = world.get::<TestComponent>(e);
        assert!(val.is_none());

        // Ensure that `get_mut` is working properly
        let val = world.get_mut::<TestComponent>(e);
        assert!(val.is_none());
    }
}
