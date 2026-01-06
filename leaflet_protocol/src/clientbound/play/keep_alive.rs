use crate::NetworkType;
use crate::Packet;
use leaflet_macros::{NetworkType, Packet};

#[derive(Debug, NetworkType, Packet)]
#[clientbound]
#[state(Configuration)]
#[packet_id(0x2B)]
pub struct ClientboundKeepAlivePacket {
    pub id: i64
}