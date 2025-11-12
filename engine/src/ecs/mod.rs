use crate::ecs::{
    scheduler::{Scheduler, System},
    world::World,
};

pub mod component;
pub mod entity;
pub mod scheduler;
pub mod storage;
pub mod world;

pub struct ECS {
    world: World,
    scheduler: Scheduler,
}

impl ECS {
    pub fn new() -> Self {
        return Self {
            world: World::new(),
            scheduler: Scheduler::new(),
        };
    }

    pub fn add_system<S>(&mut self, system: S)
    where
        S: System + 'static,
    {
        self.scheduler.add_system(system);
    }

    pub fn run_systems(&mut self, dt: std::time::Duration) {
        self.scheduler.run_systems(&mut self.world, dt);
    }
}
