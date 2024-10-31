#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [u32; 2],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        return wgpu::VertexBufferLayout {
            // How many bytes to jump to get to the next vertex
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            // Vertex buffer is advanced in every vertex
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                // Each vertex is made up of two u32, representing a 2D position
                format: wgpu::VertexFormat::Uint32x2,
                // Where to find the vertex in the buffer when configuring shader file
                shader_location: 0,
                // How many bytes need to be skipped to get this attribute. Because I have only one
                // attribute, this is always 0.
                offset: 0,
            }],
        };
    }
}
