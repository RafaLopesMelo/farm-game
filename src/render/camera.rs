use winit::{
    event::{ElementState, KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

use crate::world::camera::Camera;

pub struct CameraController {
    pub camera: Camera,
    forward: bool,
    backward: bool,
    left: bool,
    right: bool,
}

impl CameraController {
    const SPEED: u32 = 1;

    pub fn new() -> Self {
        let camera = Camera::new();

        Self {
            camera,
            forward: false,
            backward: false,
            left: false,
            right: false,
        }
    }

    pub fn handle_events(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state,
                        physical_key: PhysicalKey::Code(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = state == ElementState::Pressed;

                match keycode {
                    KeyCode::KeyW => {
                        self.forward = is_pressed;
                    }
                    KeyCode::KeyA => {
                        self.left = is_pressed;
                    }
                    KeyCode::KeyS => {
                        self.backward = is_pressed;
                    }
                    KeyCode::KeyD => {
                        self.right = is_pressed;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    pub fn update(&mut self) {
        let s = Self::SPEED as i32;
        let coords = &mut self.camera.coords;

        if self.right {
            coords.move_x(s);
        }

        if self.left {
            coords.move_x(-s);
        }

        if self.forward {
            coords.move_y(s);
        }

        if self.backward {
            coords.move_y(-s);
        }
    }
}
