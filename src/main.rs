mod app;
mod game;
mod internal;
mod render;
mod state;
mod world;

use internal::window::Window;

fn main() {
    let mut window = Window::new();
    window.run();
}
