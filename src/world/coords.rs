#[derive(Copy, Clone)]
pub struct Coords {
    coords: [i32; 2],
}

impl Coords {
    pub fn new(x: i32, y: i32) -> Self {
        return Self { coords: [x, y] };
    }

    pub fn x(&self) -> i32 {
        return self.coords[0];
    }

    pub fn y(&self) -> i32 {
        return self.coords[1];
    }

    pub fn offset(&self, other: &Coords) -> [i32; 2] {
        return [other.x() - self.x(), other.y() - self.y()];
    }

    pub fn to_array(&self) -> [i32; 2] {
        return self.coords;
    }
}
