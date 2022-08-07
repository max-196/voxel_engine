pub mod list;

use std::net::SocketAddr;

struct Client {
    address: SocketAddr,
}

impl Client {
    fn new(address: SocketAddr) -> Self {
        Self { address }
    }
}