use std::ops;
use std::marker::Copy;
use std::clone::Clone;

#[derive(Copy, Clone)]
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

    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 {
            x,
            y,
        }
    }
}

impl ops::Add for Vector2 {
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Mul<f32> for Vector2 {
    type Output = Self;
    
    fn mul(self, rhs: f32) -> Self {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}

impl ops::Mul<Vector2> for f32 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Vector2 {
        Vector2::new(rhs.x * self, rhs.y * self)
    }
}