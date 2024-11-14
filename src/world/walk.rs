pub struct WalkIntention {
    x: i32,
    y: i32,
}

impl WalkIntention {
    pub fn new(x: i32, y: i32) -> Self {
        return Self { x, y };
    }

    pub fn is_neutral(&self) -> bool {
        return self.x == 0 && self.y == 0;
    }

    pub fn x(&self) -> i32 {
        return self.x;
    }

    pub fn y(&self) -> i32 {
        return self.y;
    }
}
