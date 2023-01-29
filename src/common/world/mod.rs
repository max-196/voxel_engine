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
    std::collections::HashMap,
};

pub const FACES: [Face; 6] = [Front, Right, Back, Left, Top, Bottom];

pub struct InnerWorld<C: Chunk> {
    pub chunk_map: HashMap<ChPos, C>,
}

impl <C: Chunk> InnerWorld<C> {
    pub fn new(cap: usize) -> Self {
        Self { chunk_map: HashMap::with_capacity(cap) }
    }
}