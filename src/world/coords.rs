#[derive(Copy, Clone, Debug)]
pub struct Coords2D {
    coords: [f32; 2],
}

impl Coords2D {
    pub fn new(x: f32, y: f32) -> Self {
        return Self { coords: [x, y] };
    }

    pub fn new_lattice(x: i32, y: i32) -> Self {
        return Self::new(x as f32, y as f32);
    }

    pub fn x(&self) -> f32 {
        return self.coords[0];
    }

    pub fn lattice_x(&self) -> i32 {
        return self.x().floor() as i32;
    }

    pub fn lattice_y(&self) -> i32 {
        return self.y().floor() as i32;
    }

    pub fn y(&self) -> f32 {
        return self.coords[1];
    }

    pub fn offset(&self, other: &Coords2D) -> [f32; 2] {
        return [other.x() - self.x(), other.y() - self.y()];
    }

    pub fn to_array(&self) -> [f32; 2] {
        return self.coords;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Coords3D {
    coords: [f32; 3],
}

impl Coords3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        return Self { coords: [x, y, z] };
    }

    pub fn new_lattice(x: i32, y: i32, z: i32) -> Self {
        return Self::new(x as f32, y as f32, z as f32);
    }

    pub fn lattice_x(&self) -> i32 {
        return self.x().floor() as i32;
    }

    pub fn lattice_y(&self) -> i32 {
        return self.y().floor() as i32;
    }

    pub fn lattice_z(&self) -> i32 {
        return self.z().floor() as i32;
    }

    pub fn x(&self) -> f32 {
        return self.coords[0];
    }

    pub fn y(&self) -> f32 {
        return self.coords[1];
    }

    pub fn z(&self) -> f32 {
        return self.coords[2];
    }

    pub fn offset(&self, other: &Coords3D) -> [f32; 3] {
        return [
            other.x() - self.x(),
            other.y() - self.y(),
            other.z() - self.z(),
        ];
    }

    pub fn to_array(&self) -> [f32; 3] {
        return self.coords;
    }

    pub fn to_2d(&self) -> Coords2D {
        return Coords2D::new(self.x(), self.y());
    }
}
