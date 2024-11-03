use winit::{
    event::{ElementState, KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Camera {
    coordinate: [u32; 2],
}

impl Camera {
    pub fn new() -> Self {
        Self { coordinate: [0, 0] }
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
        if self.right {
            self.camera.coordinate[0] += Self::SPEED;
        }

        if self.left {
            if self.camera.coordinate[0] > 0 {
                self.camera.coordinate[0] -= Self::SPEED;
            }
        }

        if self.forward {
            self.camera.coordinate[1] += Self::SPEED;
        }

        if self.backward {
            if self.camera.coordinate[1] > 0 {
                self.camera.coordinate[1] -= Self::SPEED;
            }
        }
    }
}
