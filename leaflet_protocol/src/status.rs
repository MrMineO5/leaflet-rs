use crate::NetworkType;
use crate::Packet;
use leaflet_macros::{NetworkType, Packet};

// Serverbound
#[derive(Debug, NetworkType, Packet)]
#[serverbound]
#[state(Status)]
#[packet_id(0)]
pub struct ServerboundStatusRequestPacket;

#[derive(Debug, NetworkType, Packet)]
#[serverbound]
#[state(Status)]
#[packet_id(1)]
pub struct ServerboundPingPacket {
    pub payload: i64
}

// Clientbound
#[derive(Debug, NetworkType, Packet)]
#[clientbound]
#[state(Status)]
#[packet_id(0)]
pub struct ClientboundStatusResponsePacket {
    pub response_payload: String
}

#[derive(Debug, NetworkType, Packet)]
#[clientbound]
#[state(Status)]
#[packet_id(1)]
pub struct ClientboundPongPacket {
    pub payload: i64
}
