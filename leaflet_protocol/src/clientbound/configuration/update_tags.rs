use crate::NetworkType;
use crate::Packet;
use leaflet_macros::{NetworkType, Packet};
use leaflet_network_buffer::varint::VarInt;
use leaflet_types::identifier::Identifier;

#[derive(Debug, NetworkType, Packet)]
#[clientbound]
#[state(Configuration)]
#[packet_id(0x0D)]
pub struct ClientboundUpdateTagsPacket {
    pub tagged_registries: Vec<RegistryTags>,
}

#[derive(Debug, NetworkType)]
pub struct RegistryTags {
    pub registry_id: Identifier,
    pub entries: Vec<TagEntry>,
}

#[derive(Debug, NetworkType)]
pub struct TagEntry {
    pub tag_name: Identifier,
    pub entries: Vec<VarInt>
}

impl TagEntry {
    pub fn new(tag_name: Identifier, entries: Vec<VarInt>) -> Self {
        Self { tag_name, entries }
    }

    pub fn empty(tag_name: Identifier) -> Self {
        Self { tag_name, entries: vec![] }
    }
}
