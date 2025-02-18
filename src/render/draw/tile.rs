use std::collections::HashMap;

use crate::{
    render::{device::screen::Screen, textures::Texture, vertex::Vertex},
    world::{camera::Camera, tiles::Tile},
};

fn to_ndc(pixels: i32, physical_size: u32) -> f32 {
    return pixels as f32 / physical_size as f32;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TileDrawable {
    position: [f32; 2],
    texture: Texture,
}

impl TileDrawable {
    pub fn new(tile: &dyn Tile, camera: &Camera, screen: &Screen, texture: Texture) -> Self {
        let coords = tile.coords();
        let diff = camera.coords.offset(&coords.to_2d());

        let s = Self::size() as i32;
        let offset: [i32; 2] = [diff[0].floor() as i32 * s, diff[1].floor() as i32 * s];

        return TileDrawable {
            position: [
                to_ndc(offset[0], screen.width()),
                to_ndc(offset[1], screen.height()),
            ],
            texture,
        };
    }

    /// Returns the size of each tile in pixels
    pub fn size() -> u32 {
        let size: u32 = 128;
        return size;
    }
}

pub struct TileDrawCommand {
    drawables: HashMap<u32, Vec<TileDrawable>>,
}

impl TileDrawCommand {
    pub fn new() -> Self {
        return Self {
            drawables: HashMap::new(),
        };
    }

    pub fn add(&mut self, layer: u32, drawables: Vec<TileDrawable>) {
        if !self.drawables.contains_key(&layer) {
            self.drawables.insert(layer, Vec::new());
        }

        self.drawables.get_mut(&layer).unwrap().extend(drawables);
    }

    pub fn layers(&self) -> Vec<&Vec<TileDrawable>> {
        let teste: Vec<&Vec<TileDrawable>> = self.drawables.values().collect();
        return teste;
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        return wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TileDrawable>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    shader_location: 2,
                    offset: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    shader_location: 3,
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    shader_location: 4,
                    offset: (std::mem::size_of::<[f32; 2]>() + std::mem::size_of::<[f32; 2]>())
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
