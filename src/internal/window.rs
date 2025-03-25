use std::collections::HashMap;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent as WinitWindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window as WinitWindow, WindowId},
};

struct PhysicalSize {
    width: u32,
    height: u32,
}

impl PhysicalSize {
    pub fn new(width: u32, height: u32) -> Self {
        return Self { width, height };
    }

    pub fn width(&self) -> u32 {
        return self.width;
    }

    pub fn height(&self) -> u32 {
        return self.height;
    }
}

impl Default for PhysicalSize {
    fn default() -> Self {
        return Self {
            width: 0,
            height: 0,
        };
    }
}

pub trait WindowEventHandler {
    fn handle(&self, event: WindowEvent, window: &Window);
}

#[derive(Debug)]
enum InternalWindowEvent {
    Tick,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum WindowEvent {
    Tick,
}

pub struct Window {
    instance: Option<WinitWindow>,
    size: PhysicalSize,
    handlers: HashMap<WindowEvent, Vec<Box<dyn WindowEventHandler>>>,
}

impl Window {
    pub fn new() -> Self {
        return Self {
            instance: None,
            size: PhysicalSize::default(),
            handlers: HashMap::new(),
        };
    }

    pub fn size(&self) -> &PhysicalSize {
        return &self.size;
    }

    pub fn add_handler(&mut self, event: &WindowEvent, handler: Box<dyn WindowEventHandler>) {
        if !self.handlers.contains_key(&event) {
            self.handlers.insert(event.clone(), Vec::new());
        }

        self.handlers.get_mut(event).unwrap().push(handler);
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::<InternalWindowEvent>::with_user_event()
            .build()
            .unwrap();

        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        let event_loop_proxy = event_loop.create_proxy();
        std::thread::spawn(move || loop {
            let duration = std::time::Duration::from_millis(18);
            std::thread::sleep(duration);

            event_loop_proxy
                .send_event(InternalWindowEvent::Tick)
                .unwrap();
        });

        event_loop.run_app(self).unwrap();
    }
}

impl ApplicationHandler<InternalWindowEvent> for Window {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attr = WinitWindow::default_attributes();
        self.instance = Some(event_loop.create_window(attr).unwrap());
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: InternalWindowEvent) {
        let window = self.instance.as_ref().unwrap();

        match event {
            InternalWindowEvent::Tick => window.request_redraw(),
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: WinitWindowEvent,
    ) {
        match event {
            WinitWindowEvent::Resized(_) => {
                let s = self.instance.as_ref().unwrap().inner_size();
                self.size = PhysicalSize::new(s.width, s.height);
            }
            WinitWindowEvent::RedrawRequested => {
                let handlers = self.handlers.get(&WindowEvent::Tick);

                if handlers.is_none() {
                    return;
                }

                for handler in handlers.unwrap() {
                    handler.handle(WindowEvent::Tick, &self);
                }
            }
            _ => todo!(),
        }
    }
}
