use engine::{ecs::scheduler::System, Engine};

struct TestSystem {}
impl System for TestSystem {
    fn run(&mut self, world: &mut engine::ecs::world::World, dt: std::time::Duration) {
        println!("Hello World!");
    }
}

pub fn main() {
    let mut engine = Engine::new();

    engine.run();

    engine.load_texture("../../assets/atlas.png");

    engine.add_system(TestSystem {});
}
