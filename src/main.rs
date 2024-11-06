mod app;
mod game;
mod render;
mod repositories;
mod state;
mod world;

fn main() {
    let event_loop = winit::event_loop::EventLoop::<app::UserEvent>::with_user_event()
        .build()
        .unwrap();

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let event_loop_proxy = event_loop.create_proxy();
    std::thread::spawn(move || loop {
        let duration = std::time::Duration::from_millis(18);
        std::thread::sleep(duration);

        event_loop_proxy.send_event(app::UserEvent::TIMER).unwrap();
    });

    let mut default_app = app::APP::default();
    event_loop.run_app(&mut default_app).unwrap();
}
