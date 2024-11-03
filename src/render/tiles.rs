use super::vertex::Vertex;
use crate::world::tiles::Tile;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TileRender {
    coords: [u32; 2],
    position: [u32; 2],
    kind: u32,
}

impl TileRender {
    pub fn new(tile: &Tile) -> Self {
        let coords: [u32; 2] = tile.coords();
        let position: [u32; 2] = [coords[0] * Self::size(), coords[1] * Self::size()];

        return Self {
            coords,
            position,
            kind: tile.kind() as u32,
        };
    }

    /// Size of a tile in pixels
    pub fn size() -> u32 {
        let size: u32 = 16;
        return size;
    }

    pub fn vertices() -> [Vertex; 6] {
        let size = Self::size();

        let vertices: [Vertex; 6] = [
            Vertex { position: [0, 0] },
            Vertex {
                position: [size, 0],
            },
            Vertex {
                position: [size, size],
            },
            Vertex { position: [0, 0] },
            Vertex {
                position: [0, size],
            },
            Vertex {
                position: [size, size],
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
                    format: wgpu::VertexFormat::Uint32x2,
                    shader_location: 1,
                    offset: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Uint32x2,
                    shader_location: 2,
                    offset: std::mem::size_of::<[u32; 2]>() as wgpu::BufferAddress,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Uint32,
                    shader_location: 3,
                    offset: std::mem::size_of::<[u32; 4]>() as wgpu::BufferAddress,
                },
            ],
        };
    }
}
