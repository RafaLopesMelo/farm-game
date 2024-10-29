use crate::vertex::Vertex;

/// Size of a tile in pixels
pub const SIZE: u32 = 16;

#[derive(Debug, Clone, Copy)]
pub struct Tile {}

impl Tile {
    pub fn vertices() -> [Vertex; 6] {
        let vertices: [Vertex; 6] = [
            Vertex { position: [0, 0] },
            Vertex {
                position: [SIZE, 0],
            },
            Vertex {
                position: [SIZE, SIZE],
            },
            Vertex { position: [0, 0] },
            Vertex {
                position: [0, SIZE],
            },
            Vertex {
                position: [SIZE, SIZE],
            },
        ];

        return vertices;
    }
}
