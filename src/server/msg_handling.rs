use {
    std::net::SocketAddr,
    crate::{
        client::net::CMsgType::*,
        server::{Server, net::SMsg},
    }
};

impl Server {
    pub fn handle_client_messages(&mut self) {
        while let Some((msg, sender)) = self.networking.recv() {
            match msg.get_type() {
                ConnectionRequest => self.connection_request(sender),
            }
        }
    }

    fn connection_request(&mut self, sender: SocketAddr) {
        let id = self.client_list.add_client(sender);
        self.networking.send(SMsg::connection_accept(id), sender);
    }
}