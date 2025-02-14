use crate::{
    render::{device::screen::Screen, vertex::Vertex},
    world::{camera::Camera, tiles::Tile},
};

fn to_ndc(pixels: i32, physical_size: u32) -> f32 {
    return pixels as f32 / physical_size as f32;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct TileDrawable {
    position: [f32; 2],
}

impl TileDrawable {
    pub fn new(tile: &dyn Tile, camera: &Camera, screen: &Screen) -> Self {
        let coords = tile.coords();
        let diff = camera.coords.offset(&coords.to_2d());

        let s = Self::size() as i32;
        let offset: [i32; 2] = [diff[0].floor() as i32 * s, diff[1].floor() as i32 * s];

        return TileDrawable {
            position: [
                to_ndc(offset[0], screen.width()),
                to_ndc(offset[1], screen.height()),
            ],
        };
    }

    /// Returns the size of each tile in pixels
    pub fn size() -> u32 {
        let size: u32 = 64;
        return size;
    }
}

struct TileDrawCommand {}

impl TileDrawCommand {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        return wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TileDrawable>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    shader_location: 2,
                    offset: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Sint32x2,
                    shader_location: 3,
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    shader_location: 4,
                    offset: (std::mem::size_of::<[f32; 3]>()
                        + std::mem::size_of::<[i32; 2]>()
                        + std::mem::size_of::<u32>())
                        as wgpu::BufferAddress,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    shader_location: 5,
                    offset: (std::mem::size_of::<[f32; 3]>()
                        + std::mem::size_of::<[i32; 2]>()
                        + std::mem::size_of::<u32>()
                        + std::mem::size_of::<[f32; 2]>())
                        as wgpu::BufferAddress,
                },
            ],
        };
    }

    pub fn vertices(screen: &Screen) -> [Vertex; 6] {
        let size = TileDrawable::size();

        let x = to_ndc(size as i32, screen.width());
        let y = to_ndc(size as i32, screen.height());

        let vertices: [Vertex; 6] = [
            Vertex {
                position: [0.0, 0.0],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [x, 0.0],
                uv: [1.0, 0.0],
            },
            Vertex {
                position: [x, y],
                uv: [1.0, 1.0],
            },
            Vertex {
                position: [0.0, 0.0],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [0.0, y],
                uv: [0.0, 1.0],
            },
            Vertex {
                position: [x, y],
                uv: [1.0, 1.0],
            },
        ];

        return vertices;
    }
}
