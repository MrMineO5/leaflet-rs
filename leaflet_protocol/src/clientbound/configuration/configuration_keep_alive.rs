use crate::NetworkType;
use crate::Packet;
use leaflet_macros::{NetworkType, Packet};

#[derive(Debug, NetworkType, Packet)]
#[clientbound]
#[state(Configuration)]
#[packet_id(4)]
pub struct ClientboundConfigurationKeepAlivePacket {
    pub id: i64
}