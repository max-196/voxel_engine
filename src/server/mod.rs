pub mod net;

mod err;
mod msg_handling;
mod client;
mod world;

mod common;
use common::*;

use {
    std::net::SocketAddr,
    client::list::ClientList,
    world::ServerWorld,
};

pub struct Server {
    networking: Net,
    client_list: ClientList,
    world: ServerWorld,
}

impl Server {
    pub fn new() -> Result<(Self, SocketAddr), err::ServerInitError> {
        let (networking, address) = Net::new()?;
        let client_list = ClientList::new();
        let world = ServerWorld::new();

        Ok((Self {
                networking,
                client_list,
                world,
            },
            address
        ))
    }

    pub fn update(&mut self) {
        self.handle_client_messages();
    }
}
