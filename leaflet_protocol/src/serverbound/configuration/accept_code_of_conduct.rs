use crate::Packet;
use crate::NetworkType;
use leaflet_macros::{NetworkType, Packet};

#[derive(Debug, NetworkType, Packet)]
#[serverbound]
#[state(Configuration)]
#[packet_id(9)]
pub struct ServerboundAcceptCodeOfConductPacket;
