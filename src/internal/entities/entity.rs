use std::sync::atomic::{AtomicU64, Ordering};

pub struct EntityID(u64);

impl EntityID {
    pub fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        return EntityID(NEXT_ID.fetch_add(1, Ordering::Relaxed));
    }
}

pub struct Entity {
    pub id: EntityID,
}
