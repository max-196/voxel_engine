use {
    std::{collections::HashMap, net::SocketAddr},
    super::Client,
    crate::common::world::WorldPos,
};

pub struct ClientList {
    list: HashMap<u8, Client>,
    cur_id: u8,
}

impl ClientList {
    pub fn new() -> Self {
        Self {
            list: HashMap::with_capacity(4),
            cur_id: 0,
        }
    }

    pub fn add_client(&mut self, address: SocketAddr) -> u8 {
        let id = self.cur_id;
        self.list.insert(id, Client::new(address));
        self.cur_id += 1;
        id
    }

    pub fn set_client_position(&mut self, id: u8, pos: WorldPos) {
        if let Some(client) = self.list.get_mut(&id) {
            client.pos = pos;
        }
    }
}