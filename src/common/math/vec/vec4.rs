#[derive(Clone, Copy)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl <T> Vec4<T> {
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}

impl Vec4<f32> {
    pub fn mag(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn norm(&self) -> Self {
        *self / self.mag()
    }

    pub fn zero() -> Self {
        Self::new(0., 0., 0., 0.)
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x +
        self.y * other.y +
        self.z * other.z +
        self.w * other.w
    }
}

// ADDITION
use std::ops::Add;
impl <T: Add<Output = T> + Copy> Add<Self> for Vec4<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
    }
}
// ADDITION


// MULTIPLICATION
use std::ops::Mul;
impl <T: Mul<Output = T> + Copy> Mul<T> for Vec4<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}
// MULTIPLICATION

// DIVISION
use std::ops::Div;
impl <T: Div<Output = T> + Copy> Div<T> for Vec4<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}
// DIVISION

impl <T> From<Vec4<T>> for [T; 4] {
    fn from(vec: Vec4<T>) -> Self {
        [vec.x, vec.y, vec.z, vec.w]
    }
}