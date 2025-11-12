use std::{collections::HashMap, hash::Hash};

use crate::assets::{sprite::Sprite, texture::Texture};

pub mod sprite;
pub mod texture;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpriteId(pub u16);

impl std::fmt::Display for SpriteId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Handle<T> {
    id: AssetId,
    /// This prevents compiler from complaining about the unused generic parameter `T`
    marker: std::marker::PhantomData<T>,
}

impl<T> Handle<T> {
    pub fn new(id: AssetId) -> Self {
        return Self {
            id,
            marker: std::marker::PhantomData,
        };
    }

    pub fn id(&self) -> AssetId {
        return self.id;
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct AssetId(u16);

impl AssetId {
    pub fn new(v: u16) -> Self {
        return AssetId(v);
    }

    pub fn value(&self) -> u16 {
        return self.0;
    }
}

pub struct AssetsRegistry {
    textures: HashMap<AssetId, Texture>,
    sprites: HashMap<AssetId, Sprite>,

    next_id: AssetId,
}

impl AssetsRegistry {
    pub fn new() -> Self {
        return Self {
            textures: HashMap::new(),
            sprites: HashMap::new(),
            next_id: AssetId::new(0),
        };
    }

    pub fn insert_texture(&mut self, texture: Texture) -> AssetId {
        let id = self.next_id;
        self.next_id = AssetId(id.value() + 1);
        self.textures.insert(id, texture);

        return id;
    }
}
