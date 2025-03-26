mod app;
mod game;
mod internal;
mod render;
mod state;
mod world;

use internal::app::App;

fn main() {
    App::new().run();
}
