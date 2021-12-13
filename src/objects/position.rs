#[derive(Copy, Clone)]
pub struct Position {
    bottom: f32,
    top: f32,
    left: f32,
    right: f32,
}

impl Position {
    pub fn new(bottom: f32, top: f32, left: f32, right: f32) -> Self {
        Self {
            bottom,
            top,
            left,
            right,
        }
    }
    pub fn get_left(&self) -> f32 {
        self.left
    }
    pub fn get_right(&self) -> f32 {
        self.right
    }
    pub fn get_bottom(&self) -> f32 {
        self.bottom
    }
    pub fn get_top(&self) -> f32 {
        self.top
    }
}
