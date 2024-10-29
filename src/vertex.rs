use super::block::CHUNK_SIZE;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        return wgpu::VertexBufferLayout {
            // How many bytes to jump to get to the next vertex
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            // Vertex buffer is advanced in every vertex
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                // Each vertex is made up of two f32s, representing a 2D position
                format: wgpu::VertexFormat::Float32x2,
                // Where to find the vertex in the buffer when configuring shader file
                shader_location: 0,
                // How many bytes need to be skipped to get this attribute. Because I have only one
                // attribute, this is always 0.
                offset: 0,
            }],
        };
    }
}

pub const QUAD_VERTICES: [Vertex; 6] = [
    Vertex {
        position: [0.0, 0.0],
    },
    Vertex {
        position: [16.0, 16.0],
    },
    Vertex {
        position: [16.0, 0.0],
    },
    Vertex {
        position: [0.0, 0.0],
    },
    Vertex {
        position: [0.0, 16.0],
    },
    Vertex {
        position: [16.0, 16.0],
    },
];

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    position: [f32; 2],
}

impl Instance {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        return wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                shader_location: 1,
                offset: 0,
            }],
        };
    }
}

pub fn create_instance_data(map: &super::block::Map) -> Vec<Instance> {
    let mut instances = Vec::new();

    // Discovers the offset of the current block. If the block is in the N chunk, I
    // discover the offset with the operation N * CHUNK_SIZE. After that, if the block is
    // the M, I discover the offset with the operation (N * CHUNK_SIZE) + M.
    let position = [0.0, 0.0];
    instances.push(Instance { position });

    return instances;
}
