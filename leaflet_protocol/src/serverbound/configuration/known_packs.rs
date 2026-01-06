use crate::NetworkType;
use crate::Packet;
use leaflet_macros::{NetworkType, Packet};
use crate::clientbound::configuration::known_packs::KnownPack; // TODO: Put this in a common location?

#[derive(Debug, NetworkType, Packet)]
#[clientbound]
#[state(Configuration)]
#[packet_id(0x07)]
pub struct ServerboundKnownPacksPacket {
    pub known_packs: Vec<KnownPack>
}
