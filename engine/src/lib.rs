use winit::event_loop::EventLoop;

use crate::app::App;

mod app;
mod camera;
mod ecs;
mod math;
mod renderer;
mod sprite;
mod state;
mod texture;
mod window;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        return wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        };
    }
}
const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-64.0, -64.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [64.0, -64.0, 0.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [64.0, 64.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [-64.0, 64.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
];

const INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

pub fn run() {
    let event_loop = EventLoop::with_user_event().build().unwrap();
    let mut app = App::new(&event_loop);

    event_loop.run_app(&mut app).unwrap();
}
