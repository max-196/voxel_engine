pub mod face;

pub use face::Face;

#[derive(Clone, Copy, PartialEq)]
pub enum Block {
    Air,
    Grass,
}

impl From<u8> for Block {
    fn from(s: u8) -> Self {
        match s {
            0 => Self::Air,
            1 => Self::Grass,
            _ => panic!()
        }
    }
}