#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [u32; 2],
    pub uv: [f32; 2],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        return wgpu::VertexBufferLayout {
            // How many bytes to jump to get to the next vertex
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            // Vertex buffer is advanced in every vertex
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Uint32x2,
                    shader_location: 0,
                    offset: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    shader_location: 1,
                    offset: std::mem::size_of::<[u32; 2]>() as wgpu::BufferAddress,
                },
            ],
        };
    }
}
