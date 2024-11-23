use super::coords::Coords2D;

#[derive(Copy, Clone)]
pub struct Camera {
    pub coords: Coords2D,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            coords: Coords2D::new_lattice(0, 0),
        }
    }

    pub fn coords_ref(&self) -> &Coords2D {
        return &self.coords;
    }

    pub fn coords(&self) -> Coords2D {
        return self.coords;
    }

    pub fn move_to(&mut self, coords: Coords2D) {
        self.coords = coords;
    }

    pub fn to_array(&self) -> [f32; 2] {
        return [self.coords.x(), self.coords.y()];
    }
}
