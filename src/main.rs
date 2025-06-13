mod internal;

use internal::engine::Engine;

fn main() {
    println!("123");
    Engine::new().run();
    println!("321");
}

struct FakeRenderPipeline {}

impl FakeRenderPipeline {}
