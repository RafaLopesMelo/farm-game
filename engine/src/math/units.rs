use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Pixels(f32);

impl Pixels {
    pub fn new(value: f32) -> Self {
        return Pixels(value);
    }

    pub fn value(&self) -> f32 {
        return self.0;
    }

    pub fn round(&self) -> i32 {
        return self.0.round() as i32;
    }

    pub fn floor(&self) -> i32 {
        return self.0.floor() as i32;
    }

    pub fn ceil(&self) -> i32 {
        return self.0.ceil() as i32;
    }
}

// Implement arithmetic operations
impl Add for Pixels {
    type Output = Pixels;
    fn add(self, other: Pixels) -> Pixels {
        Pixels(self.0 + other.0)
    }
}

impl Sub for Pixels {
    type Output = Pixels;
    fn sub(self, other: Pixels) -> Pixels {
        Pixels(self.0 - other.0)
    }
}

impl Mul<f32> for Pixels {
    type Output = Pixels;
    fn mul(self, scalar: f32) -> Pixels {
        Pixels(self.0 * scalar)
    }
}

impl Mul<Pixels> for f32 {
    type Output = Pixels;
    fn mul(self, pixels: Pixels) -> Pixels {
        Pixels(self * pixels.0)
    }
}

impl Div<f32> for Pixels {
    type Output = Pixels;
    fn div(self, scalar: f32) -> Pixels {
        Pixels(self.0 / scalar)
    }
}

impl Div<Pixels> for Pixels {
    type Output = f32;
    fn div(self, other: Pixels) -> f32 {
        self.0 / other.0
    }
}
