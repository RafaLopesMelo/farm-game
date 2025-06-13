use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent as WinitWindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window as WinitWindow, WindowId},
};

pub struct PhysicalSize {
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

#[derive(Debug)]
enum InternalWindowEvent {
    Tick,
}
pub struct Window {
    running: bool,
    instance: Option<Arc<WinitWindow>>,
    size: PhysicalSize,
}

impl Window {
    pub fn new() -> Self {
        return Self {
            running: false,
            instance: None,
            size: PhysicalSize::default(),
        };
    }

    pub fn size(&self) -> &PhysicalSize {
        return &self.size;
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

    pub fn instance(&self) -> Option<Arc<WinitWindow>> {
        return self.instance.clone();
    }

    pub fn running(&self) -> bool {
        return self.running;
    }
}

impl ApplicationHandler<InternalWindowEvent> for Window {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attr = WinitWindow::default_attributes();
        let w = event_loop.create_window(attr).unwrap();
        self.instance = Some(Arc::new(w));

        self.running = true;
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
            WinitWindowEvent::RedrawRequested => {}
            _ => {}
        }
    }
}
