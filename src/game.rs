use crate::world::{camera::Camera, coords::Coords, world::World};

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
    pub fn perform_movement(&mut self, movement: [i32; 2]) -> Coords {
        let coords = self.camera.perform_movement(movement);
        self.world.load(&self.camera);

        return coords;
    }
}
