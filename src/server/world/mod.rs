mod chunk;

use {
    chunk::ServerChunk,
    crate::common::{
        world::{pos::ChPos, InnerWorld},
        math::Pnt3
    },
    std::collections::HashMap,
};

pub struct ServerWorld {
    pub world: InnerWorld<ServerChunk>,
}

impl ServerWorld {
    pub fn new() -> Self {
        let mut world = InnerWorld::new(256);

        for x in -4..4 {
            for y in -4..4 {
                for z in -4..4 {
                    let chpos = ChPos::new(Pnt3::new(x, y, z));
                    let mut chunk = ServerChunk::new(chpos);
                    world.chunk_map.insert(chpos, chunk);
                }
            }
        }

        Self { world }
    }
}
