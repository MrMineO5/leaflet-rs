use leaflet_network_buffer::McBuf;
use leaflet_protocol::status::{ClientboundPongPacket, ClientboundStatusResponsePacket, ServerboundPingPacket, ServerboundStatusRequestPacket};
use leaflet_protocol::StatusServerboundHandler;
use crate::client_connection::ClientConnection;
use crate::handlers::PacketHandler;

impl StatusServerboundHandler for PacketHandler {
    type ClientType = ClientConnection;

    fn on_status_request(
        &self,
        connection: &mut Self::ClientType,
        _packet: ServerboundStatusRequestPacket,
    ) {
        let response_packet = ClientboundStatusResponsePacket {
            response_payload: "{\"version\":{\"name\":\"1.21.11\",\"protocol\":774},\"description\":{\"text\":\"Hello, world!\"}}".to_string()
        };

        connection.queue_packet(&response_packet);
    }

    fn on_ping(&self, connection: &mut Self::ClientType, packet: ServerboundPingPacket) {
        let response_packet = ClientboundPongPacket {
            payload: packet.payload,
        };
        connection.queue_packet(&response_packet);
    }

    fn on_unknown(&self, connection: &mut Self::ClientType, id: i32, buf: &mut McBuf) {
        let username = connection
            .gameprofile
            .as_ref()
            .map(|g| g.username.clone())
            .unwrap_or("unknown player".into());
        println!(
            "({username}) Received unknown Status packet: {id} length {}",
            buf.length()
        )
    }
}
