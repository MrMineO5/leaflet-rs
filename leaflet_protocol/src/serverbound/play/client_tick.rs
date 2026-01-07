use crate::NetworkType;
use crate::Packet;
use leaflet_macros::{NetworkType, Packet};

#[derive(Debug, NetworkType, Packet)]
#[serverbound]
#[state(Play)]
#[packet_id(0x0C)]
pub struct ClientboundPlayLoginPacket;
