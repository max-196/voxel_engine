#[derive(Clone, Copy)]
pub struct Angle(f32);

impl Angle {
    pub fn rad(&self) -> f32 { self.0 }
    pub fn deg(&self) -> f32 { self.0.to_degrees() }
    pub fn from_rad(r: f32) -> Self { Self(r) }
    pub fn from_deg(d: f32) -> Self { Self(d.to_radians()) }

    pub fn sin(&self) -> f32 { self.0.sin() }
    pub fn cos(&self) -> f32 { self.0.cos() }
    pub fn sin_cos(&self) -> (f32, f32) { self.0.sin_cos() }
}

use std::ops::Add;
impl Add<Self> for Angle {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

use std::ops::AddAssign;
impl AddAssign<Self> for Angle {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

use std::ops::Sub;
impl Sub<Self> for Angle {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

use std::ops::SubAssign;
impl SubAssign<Self> for Angle {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

use std::ops::Mul;
impl Mul<f32> for Angle {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}