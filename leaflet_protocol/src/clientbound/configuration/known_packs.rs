use crate::NetworkType;
use crate::Packet;
use leaflet_macros::{NetworkType, Packet};


#[derive(Debug, NetworkType)]
pub struct KnownPack {
    pub namespace: String,
    pub path: String,
    pub version: String
}
#[derive(Debug, NetworkType, Packet)]
#[clientbound]
#[state(Configuration)]
#[packet_id(0x0E)]
pub struct ClientboundKnownPacksPacket {
    pub known_packs: Vec<KnownPack>
}
