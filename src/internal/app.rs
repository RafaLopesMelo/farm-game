use super::{graphics::Graphics, window::Window};

pub trait Plugin {
    fn register(&self, app: &mut App);
}

pub struct App {
    window: Window,
    graphics: Option<Graphics>,
}

impl App {
    pub fn new() -> Self {
        return Self {
            window: Window::new(),
            graphics: None,
        };
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        plugin.register(self);
    }

    pub fn run(&mut self) {
        self.window.run();

        while !self.window.running() {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        self.graphics = Some(Graphics::new(&self.window));
    }
}
