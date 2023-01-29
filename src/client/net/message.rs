use crate::common::world::WorldPos;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum CMsgType { // C for Client
    ConnectionRequest = 0,
    Position,
}

use CMsgType::*;

impl From<u8> for CMsgType {
    fn from(s: u8) -> Self {
        match s {
            0 => ConnectionRequest,
            1 => Position,
            _ => panic!()
        }
    }
}

pub struct CMsg(pub Vec<u8>); // C for Client

impl CMsg {
    pub fn get_type(&self) -> CMsgType {
        self.0[0].into()
    }

    pub fn data(self) -> Vec<u8> {
        self.0
    }

    pub fn connection_request() -> Self {
        let buf = vec![ConnectionRequest as u8];
        Self(buf)
    }

    pub fn position(pos: &WorldPos, id: u8) -> Self {
        let mut buf = Vec::with_capacity(26);
        buf.push(Position as u8);
        buf.push(id);
        buf.extend_from_slice(&pos.to_be_bytes());
        Self(buf)
    }
}