use crate::tilemap;

pub struct TileChunkMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,

    pub chunk_coords: (i32, i32),

    pub dirty: bool,
}

impl TileChunkMesh {
    pub fn from_chunk(chunk: tilemap::Chunk) {}
}
