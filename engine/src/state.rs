use std::sync::Arc;

use winit::window::Window;

pub struct State {
    window: Arc<Window>,
}

impl State {
    pub async fn new(window: Arc<Window>) -> Result<Self, ()> {
        return Ok(Self { window });
    }

    pub fn resize(&mut self, width: u32, height: u32) {}

    pub fn render(&mut self) {
        self.window.request_redraw();
    }
}
