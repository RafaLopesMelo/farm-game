use std::any::Any;

use crate::ecs::entity::Entity;

pub trait Component: 'static + Send + Sync {}

pub trait ComponentStore {
    fn as_any_ref(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn remove(&mut self, entity: Entity);
}
