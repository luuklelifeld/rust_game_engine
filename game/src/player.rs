pub struct Player {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, speed: 80.0 }
    }
}
