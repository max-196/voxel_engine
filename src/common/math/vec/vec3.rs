use super::Pnt3;

#[derive(Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl <T> Vec3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl Vec3<f32> {
    pub fn mag(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn norm(&self) -> Self {
        *self / self.mag()
    }

    pub fn zero() -> Self {
        Self::new(0., 0., 0.)
    }

    pub fn unit_y() -> Self {
        Self::new(0., 1., 0.)
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x +
        self.y * other.y +
        self.z * other.z
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

// ADDITION
use std::ops::Add;
impl <T: Add<Output = T> + Copy> Add<Self> for Vec3<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
// ADDITION


// MULTIPLICATION
use std::ops::Mul;
impl <T: Mul<Output = T> + Copy> Mul<T> for Vec3<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
// MULTIPLICATION

// DIVISION
use std::ops::Div;
impl <T: Div<Output = T> + Copy> Div<T> for Vec3<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}
// DIVISION

impl <T> From<Pnt3<T>> for Vec3<T> {
    fn from(pnt: Pnt3<T>) -> Self {
        Self::new(pnt.x, pnt.y, pnt.z)
    }
}