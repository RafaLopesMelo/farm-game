/// `Entity` represents a thing that exists in the game world - like a player or a tree
///
/// In ECS-based engines, the `Entity` does not store data itself. Instead, it is a unique
/// identifier that refers to a collection of data called `Component` stored elsewhere
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Entity {
    slot: u32,
    generation: u32,
}

impl Entity {
    pub fn new(slot: u32, generation: u32) -> Self {
        return Self { slot, generation };
    }

    pub fn slot(&self) -> u32 {
        return self.slot;
    }

    pub fn generation(&self) -> u32 {
        return self.generation;
    }

    pub(crate) fn idx(&self) -> usize {
        return self.slot as usize;
    }
}

pub struct EntityAllocator {
    next: u32,
    recycled: Vec<u32>,
    generations: Vec<u32>,
}

impl EntityAllocator {
    pub fn new() -> Self {
        return Self {
            next: 0,
            recycled: Vec::new(),
            generations: Vec::new(),
        };
    }

    pub fn alloc(&mut self) -> Entity {
        if let Some(i) = self.recycled.pop() {
            let idx = i as usize;
            return Entity::new(i, self.generations[idx]);
        }

        let slot = self.next;
        let generation = 0;

        self.generations.push(generation);
        self.next += 1;

        return Entity::new(slot, generation);
    }

    pub fn free(&mut self, entity: Entity) {
        let idx = entity.idx();

        if idx >= self.generations.len() {
            panic!("Entity index out of the bounds");
        }

        if self.generations[idx] != entity.generation() {
            panic!("entity generation mismatch - already freed");
        }

        self.recycled.push(entity.slot());
        self.generations[idx] = self.generations[idx].wrapping_add(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alloc_new() {
        let mut a = EntityAllocator::new();
        let e = a.alloc();

        assert_eq!(e.slot(), 0);
        assert_eq!(e.generation(), 0);
    }

    #[test]
    fn test_multi_alloc() {
        let mut a = EntityAllocator::new();

        a.alloc();
        let e2 = a.alloc();

        assert_eq!(e2.slot(), 1);
        assert_eq!(e2.generation(), 0);
    }

    #[test]
    fn test_alloc_recycling() {
        let mut a = EntityAllocator::new();

        let e1 = a.alloc();
        a.free(e1);

        let e2 = a.alloc();

        assert_eq!(e2.slot(), 0);
        assert_eq!(e2.generation(), 1);
    }
}
