use {
    std::ops::Add,
    super::{ChVec, CH_SIZE, CH_SLICE},
};

#[derive(Clone, Copy)]
pub struct ChInd(pub usize);

impl ChInd {

}

impl From<ChVec> for ChInd {
    fn from(v: ChVec) -> Self {
        ChInd(v.0.x as usize + v.0.z as usize * CH_SIZE + v.0.y as usize * CH_SLICE)
    }
}

impl Add<ChVec> for ChInd {
    type Output = Self;
    fn add(self, rhs: ChVec) -> Self::Output {
        (ChVec::from(self) + rhs).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::Face;

    #[test]
    fn conversion() {
        assert_eq!(ChInd::from(ChVec::new(5, 0, 0)).0, 5);
        assert_eq!(ChInd::from(ChVec::new(0, 1, 0)).0, CH_SLICE);
        assert_eq!(ChInd::from(ChVec::new(0, 0, 1)).0, CH_SIZE);
    }

    #[test]
    fn add() {
        assert_eq!((ChInd(0) + Face::Back.into()).0, 1);
    }
}