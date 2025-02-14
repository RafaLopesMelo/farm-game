pub struct Screen {
    width: u32,
    height: u32,
}

impl Screen {
    pub fn new(width: u32, height: u32) -> Self {
        return Self { width, height };
    }

    pub fn width(&self) -> u32 {
        return self.width;
    }

    pub fn height(&self) -> u32 {
        return self.height;
    }

    pub fn update(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}
