use leaflet_network_buffer::NetworkType;
use crate::Packet;
use leaflet_macros::{NetworkType, Packet};

#[derive(Debug, NetworkType, Packet)]
#[serverbound]
#[state(Handshake)]
#[packet_id(0)]
pub struct ServerboundHandshakePacket {
    #[varint]
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    #[varint]
    pub intent: i32
}
