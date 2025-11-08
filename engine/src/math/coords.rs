use std::ops::Sub;

/// Alias for coords values constraints
/// Useful to avoid code duplication
pub trait CoordScalar: Copy + Sub<Output = Self> {}
impl<T> CoordScalar for T where T: Copy + Sub<Output = T> {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coords2D<T>
where
    T: CoordScalar,
{
    x: T,
    y: T,
}

impl<T> Coords2D<T>
where
    T: CoordScalar,
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

impl<T> Sub for Coords2D<T>
where
    T: CoordScalar,
{
    type Output = Coords2D<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        return Coords2D::new(self.x() - rhs.x(), self.y() - rhs.y());
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coords3D<T>
where
    T: CoordScalar,
{
    x: T,
    y: T,
    z: T,
}

impl<T> Coords3D<T>
where
    T: CoordScalar,
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
