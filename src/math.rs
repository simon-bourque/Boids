pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn lengthsq(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> f32 {
        self.lengthsq().sqrt()
    }

    pub fn dot(&self, v: Vector2) -> f32 {
        self.x * v.x + self.y + v.y
    }
}