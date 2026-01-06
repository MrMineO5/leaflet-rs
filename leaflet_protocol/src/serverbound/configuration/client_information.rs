use crate::NetworkType;
use crate::Packet;
use leaflet_macros::{NetworkType, Packet};

#[derive(Debug, NetworkType, Packet)]
#[serverbound]
#[state(Configuration)]
#[packet_id(0)]
pub struct ServerboundClientInformationPacket {
    pub locale: String,
    pub view_distance: i8,
    #[varint]
    pub chat_mode: i32,
    pub chat_colors: bool,
    pub displayed_skin_parts: u8,
    #[varint]
    pub main_hand: i32,
    pub enable_text_filtering: bool,
    pub allow_server_listings: bool,
    #[varint]
    pub particle_status: i32
}
