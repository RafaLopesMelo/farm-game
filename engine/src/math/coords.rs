#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coords2D<T>
where
    T: Copy,
{
    x: T,
    y: T,
}

impl<T> Coords2D<T>
where
    T: Copy,
{
    pub fn new(x: T, y: T) -> Self {
        return Self { x, y };
    }

    pub fn x(&self) -> T {
        return self.x;
    }

    pub fn y(&self) -> T {
        return self.y;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coords3D<T>
where
    T: Copy,
{
    x: T,
    y: T,
    z: T,
}

impl<T> Coords3D<T>
where
    T: Copy,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        return Self { x, y, z };
    }

    pub fn x(&self) -> T {
        return self.x;
    }

    pub fn y(&self) -> T {
        return self.y;
    }

    pub fn z(&self) -> T {
        return self.z;
    }

    pub fn to_2d(&self) -> Coords2D<T> {
        return Coords2D::new(self.x, self.y);
    }
}
