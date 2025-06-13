use std;
use std::collections::HashMap;

use super::texture::Texture;

pub struct AssetsManager {
    textures: HashMap<String, Texture>,
}

impl AssetsManager {
    pub fn new() -> Self {
        return Self {
            textures: HashMap::new(),
        };
    }

    pub fn load_texture(
        &mut self,
        name: &str,
        buffer: &[u8; 0],
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<(), AssetsManagerError> {
        if self.textures.contains_key(name) {
            return Err(AssetsManagerError::AssetAlreadyLoaded(name.to_string()));
        }

        let t = Texture::new(name, buffer, device, queue);
        self.textures.insert(name.to_string(), t);

        return Ok(());
    }

    pub fn get_texture(&self, name: &str) -> Option<&Texture> {
        return self.textures.get(name);
    }
}

#[derive(Debug)]
pub enum AssetsManagerError {
    AssetAlreadyLoaded(String),
}

impl std::fmt::Display for AssetsManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetsManagerError::AssetAlreadyLoaded(name) => {
                write!(f, "Asset already loaded: {}", name)
            }
        }
    }
}

impl std::error::Error for AssetsManagerError {}
