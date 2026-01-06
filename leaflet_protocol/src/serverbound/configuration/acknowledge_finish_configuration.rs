use crate::NetworkType;
use crate::Packet;
use leaflet_macros::{NetworkType, Packet};

#[derive(Debug, NetworkType, Packet)]
#[serverbound]
#[state(Configuration)]
#[packet_id(3)]
pub struct ServerboundAcknowledgeFinishConfigurationPacket;
