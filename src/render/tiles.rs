use super::{textures::Texture, vertex::Vertex};
use crate::world::{camera::Camera, tiles::Tile};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TileRender {
    coords: [f32; 3],
    offset: [i32; 2],
    kind: u32,
    texture: Texture,
}

impl TileRender {
    pub fn new(tile: &dyn Tile, texture: Texture, camera: &Camera) -> Self {
        let coords = &tile.coords();

        let s = Self::size() as i32;
        let diff = camera.coords.offset(&coords.to_2d());

        let offset: [i32; 2] = [diff[0].floor() as i32 * s, diff[1].floor() as i32 * s];

        return Self {
            coords: coords.to_array(),
            offset,
            kind: tile.kind() as u32,
            texture,
        };
    }

    /// Size of a tile in pixels
    pub fn size() -> u32 {
        let size: u32 = 64;
        return size;
    }

    pub fn vertices() -> [Vertex; 6] {
        let size = Self::size();

        let vertices: [Vertex; 6] = [
            Vertex {
                position: [0, 0],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [size, 0],
                uv: [1.0, 0.0],
            },
            Vertex {
                position: [size, size],
                uv: [1.0, 1.0],
            },
            Vertex {
                position: [0, 0],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [0, size],
                uv: [0.0, 1.0],
            },
            Vertex {
                position: [size, size],
                uv: [1.0, 1.0],
            },
        ];

        return vertices;
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        return wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TileRender>() as wgpu::BufferAddress,
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
                    format: wgpu::VertexFormat::Uint32,
                    shader_location: 4,
                    offset: (std::mem::size_of::<[f32; 3]>() + std::mem::size_of::<[i32; 2]>())
                        as wgpu::BufferAddress,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    shader_location: 5,
                    offset: (std::mem::size_of::<[f32; 3]>()
                        + std::mem::size_of::<[i32; 2]>()
                        + std::mem::size_of::<u32>())
                        as wgpu::BufferAddress,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    shader_location: 6,
                    offset: (std::mem::size_of::<[f32; 3]>()
                        + std::mem::size_of::<[i32; 2]>()
                        + std::mem::size_of::<u32>()
                        + std::mem::size_of::<[f32; 2]>())
                        as wgpu::BufferAddress,
                },
            ],
        };
    }
}
