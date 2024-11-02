use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

use crate::state::State;

#[derive(Debug)]
pub enum UserEvent {
    TIMER,
}

#[derive(Default)]
pub struct APP<'a> {
    window: Option<Arc<Window>>,
    state: Option<State<'a>>,
}

impl ApplicationHandler<UserEvent> for APP<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attr = Window::default_attributes();
        let window = Arc::new(event_loop.create_window(attr).unwrap());

        self.window = Some(window.clone());

        let state = pollster::block_on(State::new(window.clone()));
        self.state = Some(state);
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: UserEvent) {
        let window = self.window.as_ref().unwrap();

        match event {
            UserEvent::TIMER => {
                window.request_redraw();
            }
        }
    }

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::Resized(_) => {
                let size = self.window.as_ref().unwrap().inner_size();
                let state = self.state.as_mut().unwrap();

                state.resize(size);
            }
            WindowEvent::RedrawRequested => {
                let state = self.state.as_mut().unwrap();
                let result = state.render();
                match result {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    Err(wgpu::SurfaceError::Timeout) => {}
                    Err(wgpu::SurfaceError::Outdated) => {}
                    Err(wgpu::SurfaceError::OutOfMemory) => std::process::exit(1),
                }
            }
            _ => {
                let state = self.state.as_mut().unwrap();
                state.camera_controller.handle_events(event);
                state.update_camera();
            }
        }
    }
}
