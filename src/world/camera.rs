use super::coords::Coords;

pub struct Camera {
    pub coords: Coords,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            coords: Coords::new(0, 0),
        }
    }

    pub fn to_tuple(&self) -> [i32; 2] {
        return [self.coords.x(), self.coords.y()];
    }
}
