use crate::NetworkType;
use crate::Packet;
use leaflet_macros::{NetworkType, Packet};
use leaflet_nbt::NBTTag;
use leaflet_types::identifier::Identifier;


#[derive(Debug, NetworkType, Packet)]
#[clientbound]
#[state(Configuration)]
#[packet_id(7)]
pub struct ClientboundRegistryDataPacket {
    pub registry_id: Identifier,
    pub entries: Vec<RegistryEntry>,
}

#[derive(Debug, NetworkType)]
pub struct RegistryEntry {
    pub identifier: Identifier,
    pub data: Option<NBTTag>
}

impl RegistryEntry {
    pub fn new(identifier: Identifier, data: NBTTag) -> Self {
        Self { identifier, data: Some(data) }
    }

    pub fn empty(identifier: Identifier) -> Self {
        Self { identifier, data: None }
    }
}
