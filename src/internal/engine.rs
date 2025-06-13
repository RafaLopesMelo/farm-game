use crate::internal::events::EventBus;

use super::{
    graphics::{Graphics, GraphicsAPI},
    window::Window,
};

pub trait Plugin {
    fn register(&self, engine: &mut Engine);
}

pub struct Engine {
    window: Window,

    graphics: Graphics,
    event_bus: EventBus,
}

impl Engine {
    pub fn new() -> Self {
        return Self {
            window: Window::new(),
            graphics: Graphics::new(),
        };
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        plugin.register(self);
    }

    pub fn graphics(&self) -> GraphicsAPI<'_> {
        return GraphicsAPI::with(&self.graphics);
    }

    pub fn run(&mut self) {
        self.window.run();
    }
}
