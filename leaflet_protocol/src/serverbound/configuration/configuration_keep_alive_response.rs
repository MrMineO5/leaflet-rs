use crate::NetworkType;
use crate::Packet;
use leaflet_macros::{NetworkType, Packet};

#[derive(Debug, NetworkType, Packet)]
#[serverbound]
#[state(Configuration)]
#[packet_id(4)]
pub struct ServerboundConfigurationKeepAliveResponsePacket {
    pub id: i64
}