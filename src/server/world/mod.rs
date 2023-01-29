mod chunk;

use {
    chunk::ServerChunk,
    crate::common::{
        world::{pos::ChPos, InnerWorld},
        math::Pnt3
    }
};

pub struct ServerWorld {
    world: InnerWorld<ServerChunk>,
}

impl ServerWorld {
    pub fn new() -> Self {
        let mut chunk_list = Vec::with_capacity(4);
        for x in 0..4 {
            chunk_list.push(ServerChunk::new(ChPos::new(Pnt3::new(x, 0, 0))));
        }
        let world = InnerWorld::new(chunk_list);

        Self { world }
    }
}
