pub mod error;
pub mod message;
pub mod msg_handling;

use {
    std::net::{UdpSocket, SocketAddr},
    crate::{
        common::networking::*,
        server::net::message::SMsg,
    },
    error::NetInitError::{self, SocketError},
};

mod common;
pub use common::*;

pub struct Net {
    socket: UdpSocket,
    pub id: u8,
    buffer: Buffer,
}

impl Net {
    pub fn new(server_address: SocketAddr) -> Result<Self, NetInitError> {
        let addresses = [
            SocketAddr::from(([127, 0, 0, 1], 9100)),
            SocketAddr::from(([127, 0, 0, 1], 9101)),
            SocketAddr::from(([127, 0, 0, 1], 9102)),
            SocketAddr::from(([127, 0, 0, 1], 9103)),
        ];

        let socket = match UdpSocket::bind(&addresses[..]) {
            Ok(v) => v,
            Err(e) => return Err(error::NetInitError::SocketError(e)),
        };
        if let Err(e) = socket.set_nonblocking(true) {return Err(SocketError(e))}

        if let Err(e) =  socket.connect(server_address) {return Err(SocketError(e))}

        let buffer = buffer();

        let mut networking = Self { socket, id: 0, buffer };
        networking.send(CMsg::connection_request());

        Ok(networking)
    }

    pub fn recv(&mut self) -> Option<SMsg> {
        match self.socket.recv(&mut self.buffer) {
            Ok(size) => {
                log::debug!("{} bytes received by the client", size);
                Some(SMsg(self.buffer[..size].to_vec()))
            },
            Err(_) => {
                //log::trace!("{}", e);
                None
            },
        }
    }

    pub fn send(&mut self, message: CMsg) {
        match self.socket.send(&message.data()) {
            Ok(size) => log::debug!("{} bytes sent by the client", size),
            Err(e) => log::error!("Client encountered an error while sending: {}", e),
        }
    }

    pub fn set_id(&mut self, id: u8) {
        self.id = id;
    }
}