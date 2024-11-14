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

    pub fn coords_ref(&self) -> &Coords {
        return &self.coords;
    }

    pub fn coords(&self) -> Coords {
        return self.coords;
    }

    pub fn move_to(&mut self, coords: Coords) {
        self.coords = coords;
    }

    pub fn to_tuple(&self) -> [i32; 2] {
        return [self.coords.x(), self.coords.y()];
    }
}
