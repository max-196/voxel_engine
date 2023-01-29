pub mod list;

use std::net::SocketAddr;

use crate::common::world::WorldPos;

#[derive(Debug)]
struct Client {
    address: SocketAddr,
    pos: WorldPos,
}

impl Client {
    fn new(address: SocketAddr) -> Self {
        Self { address, pos: Default::default() }
    }
}