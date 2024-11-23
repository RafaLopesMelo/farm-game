pub struct WalkIntention {
    x: f32,
    y: f32,
}

impl WalkIntention {
    pub fn new(x: f32, y: f32) -> Self {
        return Self { x, y };
    }

    pub fn is_neutral(&self) -> bool {
        return self.x == 0.0 && self.y == 0.0;
    }

    pub fn x(&self) -> f32 {
        return self.x;
    }

    pub fn y(&self) -> f32 {
        return self.y;
    }
}
