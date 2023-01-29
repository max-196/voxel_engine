pub mod message;

use {
    std::net::{UdpSocket, SocketAddr},
    super::err::ServerInitError,
    crate::{
        common::networking::*,
        client::net::CMsg
    }
};

mod common;
pub use common::*;

pub struct Net {
    pub socket: UdpSocket,
    pub buffer: Buffer,
}

impl Net {
    pub fn new() -> Result<(Self, SocketAddr), ServerInitError> {
        let addresses = [
            SocketAddr::from(([127, 0, 0, 1], 9000)),
            SocketAddr::from(([127, 0, 0, 1], 9001)),
            SocketAddr::from(([127, 0, 0, 1], 9002)),
            SocketAddr::from(([127, 0, 0, 1], 9003)),
        ];

        let socket = match UdpSocket::bind(&addresses[..]) {
            Ok(v) => v,
            Err(e) => return Err(ServerInitError::Socket(e))
        };

        if let Err(e) = socket.set_nonblocking(true) {return Err(ServerInitError::Socket(e))}

        let address = match socket.local_addr() {
            Ok(v) => v,
            Err(e) => return Err(ServerInitError::Socket(e)),
        };

        log::info!("Server networking initialized successfully at port: {}", address);

        let buffer = buffer();

        Ok((Self { socket, buffer }, address))
    }

    pub fn recv(&mut self) -> Option<(CMsg, SocketAddr)> {
        match self.socket.recv_from(&mut self.buffer) {
            Ok((size, sender)) => {
                log::debug!("{} bytes received from {} on the server", size, sender);
                Some((CMsg(self.buffer[..size].to_vec()), sender))
            },
            Err(_) => {
                //log::trace!("{}", e);
                None
            },
        }
    }

    pub fn send(&mut self, message: SMsg, address: SocketAddr) {
        match self.socket.send_to(&message.data(), address) {
            Ok(size) => log::debug!("{} bytes sent by the server to {}", size, address),
            Err(e) => log::error!("Server encountered an error while sending to {}: {}", address, e),
        }
    }
}