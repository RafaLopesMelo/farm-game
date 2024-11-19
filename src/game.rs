use crate::world::{camera::Camera, coords::Coords2D, walk::WalkIntention, world::World};

pub struct Game {
    camera: Camera,
    world: World,
}

impl Game {
    pub fn new() -> Self {
        let camera = Camera::new();
        let mut world = World::new();

        world.load(&camera);

        return Self { camera, world };
    }

    pub fn camera_ref(&self) -> &Camera {
        return &self.camera;
    }

    pub fn world_ref(&self) -> &World {
        return &self.world;
    }

    /// Moves the camera by the given amount and returns the new coords
    pub fn perform_walk(&mut self, intention: WalkIntention) -> Coords2D {
        if intention.is_neutral() {
            return self.camera.coords();
        }

        let dest = Coords2D::new(
            self.camera.coords.x() + intention.x(),
            self.camera.coords.y() + intention.y(),
        );

        let dest_chunk = self
            .world
            .chunk_at(dest)
            .expect("chunk to move is not loaded");

        let dest_tile = dest_chunk
            .tile_at(dest)
            .expect("tile to move is not loaded");

        if !dest_tile.walkable() {
            return self.camera.coords;
        }

        self.camera.move_to(dest);
        self.world.load(&self.camera);

        return dest;
    }
}
