use winit::{
    event::{ElementState, KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

pub struct CameraController {
    forward: bool,
    backward: bool,
    left: bool,
    right: bool,
}

impl CameraController {
    const SPEED: u32 = 1;

    pub fn new() -> Self {
        Self {
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

    pub fn build_movement(&mut self) -> [i32; 2] {
        let mut movement: [i32; 2] = [0, 0];
        let s = Self::SPEED as i32;

        if self.right {
            movement[0] += s;
        }

        if self.left {
            movement[0] -= s;
        }

        if self.forward {
            movement[1] += s;
        }

        if self.backward {
            movement[1] -= s;
        }

        return movement;
    }
}
