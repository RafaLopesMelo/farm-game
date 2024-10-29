pub const CHUNK_SIZE: u32 = 32;

#[derive(Copy, Clone)]
enum BlockKind {
    Grass,
}

#[derive(Copy, Clone)]
struct Block {
    kind: BlockKind,
}

impl Block {
    fn new(kind: BlockKind) -> Self {
        return Self { kind };
    }
}

struct Chunk {
    blocks: [[Block; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
}

impl Chunk {
    fn new(blocks: [[Block; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]) -> Self {
        return Self { blocks };
    }
}

pub struct Map {
    pub chunks: Vec<Chunk>,
    pub width: u32,
    pub height: u32,
}

impl Map {
    pub fn new() -> Self {
        let blocks = [[Block::new(BlockKind::Grass); CHUNK_SIZE as usize]; CHUNK_SIZE as usize];
        let chunks = vec![Chunk::new(blocks)];

        return Self {
            chunks,
            width: 1,
            height: 1,
        };
    }
}
