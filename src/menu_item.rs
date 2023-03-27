#[derive(Clone, Copy)]
pub struct MenuItem {
    pub x: f32,
    pub y: f32,
    pub size: f32,
}

impl Default for MenuItem {
    fn default() -> Self {
        Self::new()
    }
}

impl MenuItem {
    pub fn new() -> Self {
        MenuItem {
            x: 0.0,
            y: 0.0,
            size: 0.0,
        }
    }

    pub fn update(&mut self, x: f32, y: f32, size: f32) {
        self.x = x;
        self.y = y;
        self.size = size;
    }

    pub fn click(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.size && y >= self.y && y <= self.y + self.size
    }
}
