pub mod face;

pub use face::Face;

#[derive(Clone, Copy, PartialEq)]
pub enum Block {
    Air,
    Grass,
}