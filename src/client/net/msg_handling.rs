use crate::{
    client::Client,
    server::net::message::{SMsg, SMsgType::*},
};

impl Client {
    pub fn handle_server_messages(&mut self) {
        while let Some(msg) = self.networking.recv() {
            match msg.get_type() {
                ConnectionAccept => self.connection_accept(msg),
            }
        }
    }

    fn connection_accept(&mut self, msg: SMsg) {
        self.networking.set_id(msg.connection_accept_data());
    }
}