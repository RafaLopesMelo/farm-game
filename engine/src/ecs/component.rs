use crate::ecs::entity::Entity;

pub trait Component: 'static + Send + Sync {}

pub trait ComponentStore {
    fn remove(&mut self, entity: Entity);
}
