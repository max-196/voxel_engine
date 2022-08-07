use {
    std::ops::Add,
    super::{Face, ChInd, CH_SIZE, CH_SLICE},
    crate::common::math::Vec3,
};

#[derive(Clone, Copy)]
pub struct ChVec(pub Vec3<i32>);

impl ChVec {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self(Vec3::new(x, y, z))
    }
}

use Face::*;
impl From<Face> for ChVec {
    fn from(f: Face) -> Self {
        match f {
            Front =>  Self::new(-1,  0,  0),
            Right =>  Self::new( 0,  0,  1),
            Back =>   Self::new( 1,  0,  0),
            Left =>   Self::new( 0,  0, -1),
            Top =>    Self::new( 0,  1,  0),
            Bottom => Self::new( 0, -1,  0),
        }
    }
}

impl From<ChInd> for ChVec {
    fn from(i: ChInd) -> Self {
        Self::new((i.0 % CH_SIZE) as i32, (i.0 / CH_SLICE) as i32, ((i.0 % CH_SLICE) / CH_SIZE) as i32)
    }
}

impl Add<ChVec> for ChVec {
    type Output = Self;
    fn add(self, rhs: ChVec) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion() {
        let (v1, i1) = (ChVec::new(5, 0, 0), ChVec::from(ChInd(5)));
        let (v2, i2) = (ChVec::new(0, 1, 0), ChVec::from(ChInd(CH_SLICE)));
        let (v3, i3) = (ChVec::new(0, 0, 1), ChVec::from(ChInd(CH_SIZE)));

        // change this if i implement PartialEq
        for (v, i) in [(v1, i1), (v2, i2), (v3, i3)] {
            assert_eq!((v.0.x, v.0.y, v.0.z), (i.0.x, i.0.y, i.0.z));
        }
    }
}