use leaflet_network_buffer::McBuf;
use leaflet_protocol::login::{ClientboundLoginSuccessPacket, ServerboundLoginAcknowledgedPacket, ServerboundLoginStartPacket};
use leaflet_protocol::{ConnectionState, LoginServerboundHandler};
use leaflet_types::game_profile::GameProfile;
use crate::client_connection::ClientConnection;
use crate::handlers::PacketHandler;

impl LoginServerboundHandler for PacketHandler {
    type ClientType = ClientConnection;

    fn on_login_start(
        &self,
        connection: &mut Self::ClientType,
        packet: ServerboundLoginStartPacket,
    ) {
        println!("Login start: {} ({})", packet.username, packet.uuid);

        let gameprofile = GameProfile {
            uuid: packet.uuid,
            username: packet.username,
            properties: vec![],
        };

        connection.gameprofile = Some(gameprofile.clone());

        let response_packet = ClientboundLoginSuccessPacket { gameprofile };
        connection.queue_packet(&response_packet);
    }

    fn on_login_acknowledged(
        &self,
        connection: &mut Self::ClientType,
        _packet: ServerboundLoginAcknowledgedPacket,
    ) {
        println!("Login acknowledged!");

        connection.state = ConnectionState::Configuration;
    }

    fn on_unknown(&self, connection: &mut Self::ClientType, id: i32, buf: &mut McBuf) {
        let username = connection
            .gameprofile
            .as_ref()
            .map(|g| g.username.clone())
            .unwrap_or("unknown player".into());
        println!(
            "({username}) Received unknown Login packet: {id} length {}",
            buf.length()
        )
    }
}
