#[repr(u8)]
#[derive(Clone, Copy)]
pub enum SMsgType { // S for Server
    ConnectionAccept = 0,
}

use SMsgType::*;

impl From<u8> for SMsgType {
    fn from(s: u8) -> Self {
        match s {
            0 => ConnectionAccept,
            _ => panic!()
        }
    }
}

pub struct SMsg(pub Vec<u8>); // S for Server

impl SMsg {
    pub fn get_type(&self) -> SMsgType {
        self.0[0].into()
    }

    pub fn data(self) -> Vec<u8> {
        self.0
    }

    pub fn connection_accept(id: u8) -> Self {
        let buf = vec![ConnectionAccept as u8, id];
        Self(buf)
    }

    pub fn connection_accept_data(self) -> u8 {
        self.0[1]
    }
}