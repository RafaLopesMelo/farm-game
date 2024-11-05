use winit::{
    event::{ElementState, KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Camera {
    coords: [i32; 2],
}

impl Camera {
    pub fn new() -> Self {
        Self { coords: [0, 0] }
    }
}

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

        if self.right {
            self.camera.coords[0] += s;
        }

        if self.left {
            if self.camera.coords[0] > 0 {
                self.camera.coords[0] -= s;
            }
        }

        if self.forward {
            self.camera.coords[1] += s;
        }

        if self.backward {
            if self.camera.coords[1] > 0 {
                self.camera.coords[1] -= s;
            }
        }
    }
}
