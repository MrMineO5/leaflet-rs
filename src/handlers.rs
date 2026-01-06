use leaflet_protocol::{ConfigurationServerboundHandler, ConnectionState, HandshakeServerboundHandler, LoginServerboundHandler, PlayServerboundHandler, StatusServerboundHandler};
use leaflet_protocol::clientbound::configuration::code_of_conduct::ClientboundCodeOfConductPacket;
use leaflet_protocol::clientbound::configuration::finish_configuration::ClientboundFinishConfigurationPacket;
use leaflet_protocol::clientbound::configuration::known_packs::{ClientboundKnownPacksPacket, KnownPack};
use leaflet_protocol::clientbound::configuration::registry_data::{ClientboundRegistryDataPacket, RegistryEntry};
use leaflet_protocol::clientbound::configuration::update_tags::{ClientboundUpdateTagsPacket, RegistryTags, TagEntry};
use leaflet_protocol::clientbound::play::login::ClientboundPlayLoginPacket;
use leaflet_protocol::login::{ClientboundLoginSuccessPacket, ServerboundLoginAcknowledgedPacket, ServerboundLoginStartPacket};
use leaflet_protocol::serverbound::configuration::accept_code_of_conduct::ServerboundAcceptCodeOfConductPacket;
use leaflet_protocol::serverbound::configuration::acknowledge_finish_configuration::ServerboundAcknowledgeFinishConfigurationPacket;
use leaflet_protocol::serverbound::configuration::client_information::ServerboundClientInformationPacket;
use leaflet_protocol::serverbound::configuration::configuration_keep_alive_response::ServerboundConfigurationKeepAliveResponsePacket;
use leaflet_protocol::serverbound::configuration::known_packs::ServerboundKnownPacksPacket;
use leaflet_protocol::serverbound::handshake::serverbound_handshake::ServerboundHandshakePacket;
use leaflet_protocol::status::{ClientboundPongPacket, ClientboundStatusResponsePacket, ServerboundPingPacket, ServerboundStatusRequestPacket};
use leaflet_types::game_profile::GameProfile;
use leaflet_types::identifier::Identifier;
use crate::client_connection::ClientConnection;

pub struct PacketHandler;

impl HandshakeServerboundHandler for PacketHandler {
    type ClientType = ClientConnection;

    fn on_handshake(&self, connection: &mut Self::ClientType, packet: ServerboundHandshakePacket) {
        match packet.intent {
            1 => connection.state = ConnectionState::Status,
            2 | 3 => connection.state = ConnectionState::Login,
            _ => panic!("Invalid intent: {}", packet.intent),
        }
    }
}

impl StatusServerboundHandler for PacketHandler {
    type ClientType = ClientConnection;

    fn on_status_request(
        &self,
        connection: &mut Self::ClientType,
        _packet: ServerboundStatusRequestPacket,
    ) {
        let response_packet = ClientboundStatusResponsePacket {
            response_payload: "{\"version\":{\"name\":\"1.21.11\",\"protocol\":774},\"description\":{\"text\":\"Hello, world!\"}}".to_string()
        };

        connection.queue_packet(&response_packet);
    }

    fn on_ping(&self, connection: &mut Self::ClientType, packet: ServerboundPingPacket) {
        let response_packet = ClientboundPongPacket {
            payload: packet.payload,
        };
        connection.queue_packet(&response_packet);
    }
}

impl LoginServerboundHandler for PacketHandler {
    type ClientType = ClientConnection;

    fn on_login_start(
        &self,
        connection: &mut Self::ClientType,
        packet: ServerboundLoginStartPacket,
    ) {
        println!("Login start: {} ({})", packet.username, packet.uuid);

        let response_packet = ClientboundLoginSuccessPacket {
            gameprofile: GameProfile {
                uuid: packet.uuid,
                username: packet.username,
                properties: vec![],
            },
        };
        connection.queue_packet(&response_packet);
    }

    fn on_login_acknowledged(
        &self,
        connection: &mut Self::ClientType,
        _packet: ServerboundLoginAcknowledgedPacket,
    ) {
        println!("Login acknowledged!");

        connection.state = ConnectionState::Configuration;
    }
}

impl ConfigurationServerboundHandler for PacketHandler {
    type ClientType = ClientConnection;

    fn on_client_information(
        &self,
        connection: &mut Self::ClientType,
        packet: ServerboundClientInformationPacket,
    ) {
        println!("Client information: {:?}", packet);

        let response_packet = ClientboundKnownPacksPacket {
            known_packs: vec![KnownPack {
                namespace: "minecraft".to_string(),
                path: "core".to_string(),
                version: "1.21.11".to_string(),
            }],
        };
        connection.queue_packet(&response_packet);
    }

    fn on_known_packs(
        &self,
        connection: &mut Self::ClientType,
        packet: ServerboundKnownPacksPacket,
    ) {
        println!("Known packs: {:?}", packet);

        connection.queue_packet(&ClientboundRegistryDataPacket {
            registry_id: Identifier::minecraft("dimension_type"),
            entries: vec![
                RegistryEntry::empty(Identifier::minecraft("overworld")), // RegistryEntry::new(Identifier::minecraft("overworld"), DimensionType::overworld().to_nbt())
            ],
        });
        connection.queue_packet(&ClientboundRegistryDataPacket {
            registry_id: Identifier::minecraft("worldgen/biome"),
            entries: vec![RegistryEntry::empty(Identifier::minecraft("plains"))],
        });
        connection.queue_packet(&ClientboundRegistryDataPacket {
            registry_id: Identifier::minecraft("damage_type"),
            entries: vec![
                RegistryEntry::empty(Identifier::minecraft("cactus")),
                RegistryEntry::empty(Identifier::minecraft("campfire")),
                RegistryEntry::empty(Identifier::minecraft("cramming")),
                RegistryEntry::empty(Identifier::minecraft("dragon_breath")),
                RegistryEntry::empty(Identifier::minecraft("drown")),
                RegistryEntry::empty(Identifier::minecraft("dry_out")),
                RegistryEntry::empty(Identifier::minecraft("ender_pearl")),
                RegistryEntry::empty(Identifier::minecraft("fall")),
                RegistryEntry::empty(Identifier::minecraft("fly_into_wall")),
                RegistryEntry::empty(Identifier::minecraft("freeze")),
                RegistryEntry::empty(Identifier::minecraft("generic")),
                RegistryEntry::empty(Identifier::minecraft("generic_kill")),
                RegistryEntry::empty(Identifier::minecraft("hot_floor")),
                RegistryEntry::empty(Identifier::minecraft("in_fire")),
                RegistryEntry::empty(Identifier::minecraft("in_wall")),
                RegistryEntry::empty(Identifier::minecraft("lava")),
                RegistryEntry::empty(Identifier::minecraft("lightning_bolt")),
                RegistryEntry::empty(Identifier::minecraft("magic")),
                RegistryEntry::empty(Identifier::minecraft("on_fire")),
                RegistryEntry::empty(Identifier::minecraft("out_of_world")),
                RegistryEntry::empty(Identifier::minecraft("outside_border")),
                RegistryEntry::empty(Identifier::minecraft("stalagmite")),
                RegistryEntry::empty(Identifier::minecraft("starve")),
                RegistryEntry::empty(Identifier::minecraft("sweet_berry_bush")),
                RegistryEntry::empty(Identifier::minecraft("wither")),
            ],
        });

        connection.queue_packet(&ClientboundRegistryDataPacket {
            registry_id: Identifier::minecraft("cat_variant"),
            entries: vec![RegistryEntry::empty(Identifier::minecraft("all_black"))],
        });
        connection.queue_packet(&ClientboundRegistryDataPacket {
            registry_id: Identifier::minecraft("chicken_variant"),
            entries: vec![RegistryEntry::empty(Identifier::minecraft("warm"))],
        });
        connection.queue_packet(&ClientboundRegistryDataPacket {
            registry_id: Identifier::minecraft("cow_variant"),
            entries: vec![RegistryEntry::empty(Identifier::minecraft("warm"))],
        });
        connection.queue_packet(&ClientboundRegistryDataPacket {
            registry_id: Identifier::minecraft("frog_variant"),
            entries: vec![RegistryEntry::empty(Identifier::minecraft("warm"))],
        });
        connection.queue_packet(&ClientboundRegistryDataPacket {
            registry_id: Identifier::minecraft("painting_variant"),
            entries: vec![RegistryEntry::empty(Identifier::minecraft("kebab"))],
        });
        connection.queue_packet(&ClientboundRegistryDataPacket {
            registry_id: Identifier::minecraft("pig_variant"),
            entries: vec![RegistryEntry::empty(Identifier::minecraft("warm"))],
        });
        connection.queue_packet(&ClientboundRegistryDataPacket {
            registry_id: Identifier::minecraft("wolf_sound_variant"),
            entries: vec![RegistryEntry::empty(Identifier::minecraft("classic"))],
        });
        connection.queue_packet(&ClientboundRegistryDataPacket {
            registry_id: Identifier::minecraft("wolf_variant"),
            entries: vec![RegistryEntry::empty(Identifier::minecraft("ashen"))],
        });
        connection.queue_packet(&ClientboundRegistryDataPacket {
            registry_id: Identifier::minecraft("zombie_nautilus_variant"),
            entries: vec![RegistryEntry::empty(Identifier::minecraft("warm"))],
        });
        connection.queue_packet(&ClientboundRegistryDataPacket {
            registry_id: Identifier::minecraft("timeline"),
            entries: vec![
                RegistryEntry::empty(Identifier::minecraft("villager_schedule")),
                RegistryEntry::empty(Identifier::minecraft("early_game")),
                RegistryEntry::empty(Identifier::minecraft("day")),
                RegistryEntry::empty(Identifier::minecraft("moon")),
            ],
        });

        connection.queue_packet(&ClientboundUpdateTagsPacket {
            tagged_registries: vec![RegistryTags {
                registry_id: Identifier::minecraft("timeline"),
                entries: vec![TagEntry::empty(Identifier::minecraft(
                    "in_overworld",
                ))],
            }],
        });

        connection.queue_packet(&ClientboundCodeOfConductPacket {
            message: "By joining this server, you agree to the terms outlined in the Terms and Conditions and Privacy Policy at https://ultradev.app/terms. You further agree to the harvesting of your organs at an unspecified later date.".to_string()
        });
    }

    fn on_configuration_keep_alive_response(&self, _connection: &mut Self::ClientType, _packet: ServerboundConfigurationKeepAliveResponsePacket) {
    }

    fn on_accept_code_of_conduct(&self, connection: &mut Self::ClientType, _packet: ServerboundAcceptCodeOfConductPacket) {
        connection.queue_packet(&ClientboundFinishConfigurationPacket);
    }

    fn on_acknowledge_finish_configuration(
        &self,
        connection: &mut Self::ClientType,
        _packet: ServerboundAcknowledgeFinishConfigurationPacket,
    ) {
        println!("Acknowledge finish configuration_old!");

        connection.state = ConnectionState::Play;

        connection.queue_packet(&ClientboundPlayLoginPacket {
            entity_id: 0,
            is_hardcore: false,
            dimension_names: vec![Identifier::minecraft("overworld")],
            max_players: 20,
            view_distance: 8,
            simulation_distance: 8,
            reduced_debug_info: false,
            enable_respawn_screen: true,
            do_limited_crafting: false,
            dimension_type: 0,
            world_name: Identifier::minecraft("overworld"),
            hashed_seed: 0,
            game_mode: 0,
            previous_game_mode: -1,
            is_debug: false,
            is_flat: false,
            death_location: None,
            portal_cooldown: 0,
            sea_level: 63,
            enforces_secure_chat: false,
        })
    }
}

impl PlayServerboundHandler for PacketHandler {
    type ClientType = ClientConnection;
}

pub static HANDLERS: PacketHandler = PacketHandler;
