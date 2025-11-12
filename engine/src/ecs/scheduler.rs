use crate::ecs::world::World;

pub trait System: Send {
    fn run(&mut self, world: &mut World, dt: std::time::Duration);
}

pub struct Scheduler {
    systems: Vec<Box<dyn System>>,
}

impl Scheduler {
    pub fn new() -> Self {
        return Scheduler {
            systems: Vec::new(),
        };
    }

    pub fn add_system<S>(&mut self, system: S)
    where
        S: System + 'static,
    {
        self.systems.push(Box::new(system));
    }

    pub fn run_systems(&mut self, world: &mut World, dt: std::time::Duration) {
        for system in self.systems.iter_mut() {
            system.run(world, dt);
        }
    }
}
