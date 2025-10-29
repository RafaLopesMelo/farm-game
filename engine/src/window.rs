use winit::event_loop::EventLoop;

#[derive(Debug)]
enum WindowEvent {
    Tick,
}

pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

pub struct Window {
    w: winit::window::Window,
    config: WindowConfig,
}

impl Window {
    pub fn new(cfg: WindowConfig) {
        let event_loop = EventLoop::<WindowEvent>::with_user_event().build().unwrap();

        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        let proxy = event_loop.create_proxy();
        std::thread::spawn(move || loop {
            let duration = std::time::Duration::from_millis(18); // 60 FPS
            std::thread::sleep(duration);

            proxy.send_event(WindowEvent::Tick).unwrap();
        });
    }

    pub fn request_redraw(&self) {
        self.w.request_redraw();
    }
}
