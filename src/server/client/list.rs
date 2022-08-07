use {
    std::{collections::HashMap, net::SocketAddr},
    super::Client
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
}