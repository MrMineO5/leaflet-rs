use leaflet_network_buffer::McBuf;
use leaflet_protocol::{ConnectionState, HandshakeServerboundHandler};
use leaflet_protocol::serverbound::handshake::serverbound_handshake::ServerboundHandshakePacket;
use crate::client_connection::ClientConnection;
use crate::handlers::PacketHandler;

impl HandshakeServerboundHandler for PacketHandler {
    type ClientType = ClientConnection;

    fn on_handshake(&self, connection: &mut Self::ClientType, packet: ServerboundHandshakePacket) {
        match packet.intent {
            1 => connection.state = ConnectionState::Status,
            2 | 3 => connection.state = ConnectionState::Login,
            _ => panic!("Invalid intent: {}", packet.intent),
        }
    }

    fn on_unknown(&self, connection: &mut Self::ClientType, id: i32, buf: &mut McBuf) {
        let username = connection
            .gameprofile
            .as_ref()
            .map(|g| g.username.clone())
            .unwrap_or("unknown player".into());
        println!(
            "({username}) Received unknown Handshake packet: {id} length {}",
            buf.length()
        )
    }
}
