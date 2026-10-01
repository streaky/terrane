pub struct Point<T = f32> {
    pub x: T,
    pub y: T,
}

impl Point<f32> {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn snap(self) -> Point<u32> {
        Point {
            x: self.x.round() as u32,
            y: self.y.round() as u32,
        }
    }
}

impl Point<u32> {
    pub fn total(&self) -> u32 {
        self.x + self.y
    }
}
