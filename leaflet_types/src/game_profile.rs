use uuid::Uuid;
use leaflet_network_buffer::NetworkType;
use leaflet_macros::NetworkType;

#[derive(Debug, NetworkType)]
pub struct GameProfileProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>
}

#[derive(Debug, NetworkType)]
pub struct GameProfile {
    pub uuid: Uuid,
    pub username: String,
    pub properties: Vec<GameProfileProperty>
}