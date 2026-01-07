use crate::NetworkType;
use crate::Packet;
use leaflet_macros::{NetworkType, Packet};
use leaflet_types::identifier::Identifier;
use leaflet_types::location::Location;

#[derive(Debug, NetworkType, Packet)]
#[clientbound]
#[state(Play)]
#[packet_id(0x30)]
pub struct ClientboundPlayLoginPacket {
    pub entity_id: i32,
    pub is_hardcore: bool,
    pub dimension_names: Vec<Identifier>,
    #[varint]
    pub max_players: i32,
    #[varint]
    pub view_distance: i32,
    #[varint]
    pub simulation_distance: i32,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub do_limited_crafting: bool,
    #[varint]
    pub dimension_type: i32,
    pub world_name: Identifier,
    pub hashed_seed: i64,
    pub game_mode: u8,
    pub previous_game_mode: i8,
    pub is_debug: bool,
    pub is_flat: bool,
    pub death_location: Option<Location>,
    #[varint]
    pub portal_cooldown: i32,
    #[varint]
    pub sea_level: i32,
    pub enforces_secure_chat: bool,
}

