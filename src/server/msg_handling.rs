use crate::common::{world::ChPos, math::Pnt3};

use {
    std::net::SocketAddr,
    crate::{
        client::net::CMsgType::*,
        server::{Server, net::SMsg},
        common::world::pos::WorldPos,
    }
};

impl Server {
    pub fn handle_client_messages(&mut self) {
        while let Some((msg, sender)) = self.networking.recv() {
            match msg.get_type() {
                ConnectionRequest => self.connection_request(sender),
                Position => self.position(&msg.data()),
                ChunkRequest => self.chunk_request(&msg.data()),
            }
        }
    }

    fn connection_request(&mut self, sender: SocketAddr) {
        let id = self.client_list.add_client(sender);
        self.networking.send(SMsg::connection_accept(id), sender);
    }

    fn position(&mut self, data: &[u8]) {
        let id = data[1];
        let world_position = WorldPos::from_be_bytes(&data[2..26]);
        self.client_list.set_client_position(id, world_position);
    }

    fn chunk_request(&mut self, data: &[u8]) {
        let id = data[1];
        let client = self.client_list.list.get(&id).unwrap();
        let pos = ChPos(Pnt3::<i32>::from_be_bytes(&data[2..14]));
        let (x, y, z) = (pos.0.x, pos.0.y, pos.0.z);
        self.networking.send(SMsg::chunk(pos, &self.world.world.chunk_map.get(&pos).unwrap().inner), client.address);
        println!("Sending chunk {x} {y} {z} to client {id}");
    }
}