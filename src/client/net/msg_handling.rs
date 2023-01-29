use crate::{
    client::{Client, world::chunk::ClientChunk},
    server::net::message::{SMsg, SMsgType::*},
    common::world::InnerChunk,
};

impl Client {
    pub fn handle_server_messages(&mut self) {
        while let Some(msg) = self.networking.recv() {
            match msg.get_type() {
                ConnectionAccept => self.connection_accept(msg),
                ChunkData => self.chunk(msg),
            }
        }
    }

    fn connection_accept(&mut self, msg: SMsg) {
        self.networking.set_id(msg.data()[1]);
    }

    fn chunk(&mut self, msg: SMsg) {
        let inner = InnerChunk::from_bytes(&msg.data()[1..]);
        let pos = inner.pos;
        let mut chunk = ClientChunk::new(inner, &self.renderer.device, &self.world.ch_layout);
        chunk.mesh(&self.renderer.device);
        self.world.world.chunk_map.insert(pos, chunk);
    }
}