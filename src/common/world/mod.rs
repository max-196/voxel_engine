pub mod block;
pub mod chunk;
pub mod pos;

pub use {
    block::{
        Block,
        Face::{self, *},
    },
    pos::{WorldPos, ChPos},
    chunk::*,
};

pub const FACES: [Face; 6] = [Front, Right, Back, Left, Top, Bottom];

pub struct InnerWorld<C: Chunk> {
    pub chunk_list: Vec<C>,
}

impl <C: Chunk> InnerWorld<C> {
    pub fn new(chunk_list: Vec<C>) -> Self {
        Self {chunk_list}
    }
}