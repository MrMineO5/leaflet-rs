use leaflet_network_buffer::McBuf;
use leaflet_protocol::PlayServerboundHandler;
use crate::client_connection::ClientConnection;
use crate::handlers::PacketHandler;

impl PlayServerboundHandler for PacketHandler {
    type ClientType = ClientConnection;

    fn on_unknown(&self, connection: &mut Self::ClientType, id: i32, buf: &mut McBuf) {
        let username = connection
            .gameprofile
            .as_ref()
            .map(|g| g.username.clone())
            .unwrap_or("unknown player".into());
        println!(
            "({username}) Received unknown Play packet: {id} length {}",
            buf.length()
        )
    }
}
