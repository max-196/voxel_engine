use {
    crate::{
        common::world::{pos::ChPos, *},
    }
};

pub struct ServerChunk {
    pub inner: InnerChunk,
}

impl ServerChunk {
    pub fn new(pos: ChPos) -> Self {
        let mut blocks = [Block::Air; CH_VOL];
        for (ind, b) in blocks.iter_mut().enumerate() {
            let bp = ChVec::from(ChInd(ind));
            if bp.0.y == 0 {
                *b = Block::Grass;
            }
        }
        let inner = InnerChunk::new(pos, blocks);
        Self { inner }
    }
}

impl Chunk for ServerChunk {}
