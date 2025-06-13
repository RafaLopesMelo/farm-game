use std::hash::Hash;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Sprite {
    uv_min: [f32; 2],
    uv_max: [f32; 2],
}

impl Sprite {
    pub fn new(uv_min: [f32; 2], uv_max: [f32; 2]) -> Self {
        return Self { uv_min, uv_max };
    }
}

impl Eq for Sprite {}

impl Hash for Sprite {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.uv_min[0].to_bits().hash(state);
        self.uv_min[1].to_bits().hash(state);
        self.uv_max[0].to_bits().hash(state);
        self.uv_max[1].to_bits().hash(state);
    }
}
