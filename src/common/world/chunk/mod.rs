pub mod ind;
pub mod vec;

pub use {
    ind::ChInd,
    vec::ChVec,
};

pub const CH_SIZE: usize = 32;
pub const CH_SLICE: usize = CH_SIZE * CH_SIZE;
pub const CH_VOL: usize = CH_SLICE * CH_SIZE;

use super::{Block, ChPos, Face};
pub trait Chunk {
}

pub struct InnerChunk {
    pub pos: ChPos,
    pub blocks: [Block; CH_VOL],
}

impl InnerChunk {
    pub fn new(pos: ChPos, blocks: [Block; CH_VOL]) -> Self {
        Self { pos, blocks }
    }
}