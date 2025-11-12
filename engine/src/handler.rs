use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::PhysicalKey,
    window::Window,
};

use crate::internal::Internal;

pub const PIXELS_PER_LINE: f32 = 120.0;
pub const MAX_SCROLL_LINES: f32 = 3.0;

pub struct Handler {
    internal: Option<Internal>,
}

impl Handler {
    pub fn new() -> Self {
        return Self { internal: None };
    }

    pub fn internal(&self) -> &Internal {
        return self.internal.as_ref().unwrap();
    }

    pub fn internal_mut(&mut self) -> &mut Internal {
        return self.internal.as_mut().unwrap();
    }
}

impl ApplicationHandler<Internal> for Handler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attrs = Window::default_attributes();

        let window = Arc::new(event_loop.create_window(window_attrs).unwrap());
        self.internal = Some(pollster::block_on(Internal::new(window)).unwrap());
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: Internal) {
        self.internal = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let internal = match &mut self.internal {
            Some(s) => s,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => internal.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                internal.update();

                match internal.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = internal.window.inner_size();
                        internal.resize(size.width, size.height);
                    }
                    Err(e) => {
                        panic!("unable to render {}", e);
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => internal.handle_key(event_loop, code, key_state.is_pressed()),
            WindowEvent::MouseWheel { delta, .. } => {
                let normalized_lines = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => y,
                    winit::event::MouseScrollDelta::PixelDelta(pos) => {
                        // Convert pixel delta into approximate "line" units to keep zoom speed stable.
                        (pos.y as f32 / PIXELS_PER_LINE).clamp(-1.0, 1.0)
                    }
                };

                let clamped = normalized_lines.clamp(-MAX_SCROLL_LINES, MAX_SCROLL_LINES);

                internal.handle_wheel(clamped);
            }
            _ => {}
        }
    }
}
