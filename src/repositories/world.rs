pub struct WorldRepository {}

impl WorldRepository {
    pub fn new() -> Self {
        Self {}
    }

    /**
    Loads chunks around the given center

    # Arguments

    `center` - center of the chunk area
    `radius` - radius in number of chunks to be loaded
    */
    pub fn load_chunks(&self) -> () {
        unimplemented!();
    }
}
