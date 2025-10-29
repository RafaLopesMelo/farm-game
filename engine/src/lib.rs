use winit::event_loop::EventLoop;

use crate::app::App;

mod app;
mod state;
mod window;

pub fn run() {
    let event_loop = EventLoop::with_user_event().build().unwrap();
    let mut app = App::new(&event_loop);

    event_loop.run_app(&mut app).unwrap();
}
