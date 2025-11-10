use crate::{assets::AssetId, math::uv::UvRect};

#[derive(Debug, Clone)]
pub struct Sprite {
    texture_id: AssetId,
    uv: UvRect,
}

impl Sprite {
    pub fn new(texture_id: AssetId, uv: UvRect) -> Self {
        return Self { texture_id, uv };
    }
}
