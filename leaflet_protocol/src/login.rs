use crate::NetworkType;
use uuid::Uuid;
use crate::Packet;
use leaflet_macros::{NetworkType, Packet};
use leaflet_types::game_profile::GameProfile;

#[derive(Debug, NetworkType, Packet)]
#[serverbound]
#[state(Login)]
#[packet_id(0)]
pub struct ServerboundLoginStartPacket {
    pub username: String,
    pub uuid: Uuid
}
#[derive(Debug, NetworkType, Packet)]
#[serverbound]
#[state(Login)]
#[packet_id(3)]
pub struct ServerboundLoginAcknowledgedPacket;



#[derive(Debug, NetworkType, Packet)]
#[clientbound]
#[state(Login)]
#[packet_id(2)]
pub struct ClientboundLoginSuccessPacket {
    pub gameprofile: GameProfile
}
