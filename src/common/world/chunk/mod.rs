pub mod ind;
pub mod vec;

pub use {
    ind::ChInd,
    vec::ChVec,
};

pub const CH_SIZE: usize = 32;
pub const CH_SLICE: usize = CH_SIZE * CH_SIZE;
pub const CH_VOL: usize = CH_SLICE * CH_SIZE;

use crate::common::math::Pnt3;

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

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(CH_VOL + 12);
        buf.extend_from_slice(&self.pos.0.as_be_bytes());
        for block in self.blocks {
            buf.push(block as u8);
        }
        buf
    }

    pub fn from_bytes(src: &[u8]) -> Self {
        assert!(src.len() == (CH_VOL + 12));
        let pos = ChPos::new(Pnt3::<i32>::from_be_bytes(&src[0..12]));
        let mut blocks = [Block::Air; CH_VOL];
        for (ind, block) in src[12..(CH_VOL + 12)].iter().enumerate() {
            blocks[ind] = Block::from(*block);
        }

        Self {
            pos,
            blocks,
        }
    }
}