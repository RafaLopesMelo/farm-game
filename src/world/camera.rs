use super::coords::Coords;

#[derive(Copy, Clone)]
pub struct Camera {
    pub coords: Coords,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            coords: Coords::new(0, 0),
        }
    }

    pub fn coords(&self) -> &Coords {
        return &self.coords;
    }

    pub fn perform_movement(&mut self, movement: [i32; 2]) -> Coords {
        self.coords = Coords::new(self.coords.x() + movement[0], self.coords.y() + movement[1]);
        return self.coords;
    }

    pub fn to_tuple(&self) -> [i32; 2] {
        return [self.coords.x(), self.coords.y()];
    }
}
