use {
    crate::{
        common::world::{pos::ChPos, *},
    }
};

pub struct ServerChunk {
    inner: InnerChunk,
}

impl ServerChunk {
    pub fn new(pos: ChPos) -> Self {
        let mut blocks = [Block::Air; CH_VOL];
        let inner = InnerChunk::new(pos, blocks);
        Self { inner }
    }
}

impl Chunk for ServerChunk {}
