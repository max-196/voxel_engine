#[repr(u8)]
#[derive(Clone, Copy)]
pub enum CMsgType { // C for Client
    ConnectionRequest = 0,
}

use CMsgType::*;

impl From<u8> for CMsgType {
    fn from(s: u8) -> Self {
        match s {
            0 => ConnectionRequest,
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
}