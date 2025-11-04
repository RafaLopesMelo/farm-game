/// `Entity` represents a thing that exists in the game world - like a player or a tree
///
/// In ECS-based engines, the `Entity` does not store data itself. Instead, it is a unique
/// identifier that refers to a collection of data called `Component` stored elsewhere
#[derive(Clone, Copy)]
pub struct Entity(u32);

impl Entity {
    pub fn new(v: u32) -> Self {
        return Self(v);
    }

    pub fn value(&self) -> u32 {
        return self.0;
    }

    pub fn to_idx(&self) -> usize {
        return self.0 as usize;
    }
    pub fn equals(&self, other: &Entity) -> bool {
        return self.value() == other.value();
    }
}
