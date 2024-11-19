#[derive(Copy, Clone, Debug)]
pub struct Coords2D {
    coords: [i32; 2],
}

impl Coords2D {
    pub fn new(x: i32, y: i32) -> Self {
        return Self { coords: [x, y] };
    }

    pub fn x(&self) -> i32 {
        return self.coords[0];
    }

    pub fn y(&self) -> i32 {
        return self.coords[1];
    }

    pub fn offset(&self, other: &Coords2D) -> [i32; 2] {
        return [other.x() - self.x(), other.y() - self.y()];
    }

    pub fn to_array(&self) -> [i32; 2] {
        return self.coords;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Coords3D {
    coords: [i32; 3],
}

impl Coords3D {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        return Self { coords: [x, y, z] };
    }

    pub fn x(&self) -> i32 {
        return self.coords[0];
    }

    pub fn y(&self) -> i32 {
        return self.coords[1];
    }

    pub fn z(&self) -> i32 {
        return self.coords[2];
    }

    pub fn offset(&self, other: &Coords3D) -> [i32; 3] {
        return [
            other.x() - self.x(),
            other.y() - self.y(),
            other.z() - self.z(),
        ];
    }

    pub fn to_array(&self) -> [i32; 3] {
        return self.coords;
    }
}
