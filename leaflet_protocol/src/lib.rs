extern crate leaflet_network_buffer;
extern crate leaflet_macros;

use std::fmt::Debug;
use leaflet_macros::packet_handlers;
use leaflet_network_buffer::NetworkType;
use login::{ClientboundLoginSuccessPacket, ServerboundLoginAcknowledgedPacket, ServerboundLoginStartPacket};
use serverbound::configuration::accept_code_of_conduct::ServerboundAcceptCodeOfConductPacket;
use serverbound::configuration::acknowledge_finish_configuration::ServerboundAcknowledgeFinishConfigurationPacket;
use serverbound::configuration::client_information::ServerboundClientInformationPacket;
use serverbound::configuration::configuration_keep_alive_response::ServerboundConfigurationKeepAliveResponsePacket;
use serverbound::configuration::known_packs::ServerboundKnownPacksPacket;
use serverbound::handshake::serverbound_handshake::ServerboundHandshakePacket;
use status::{ClientboundPongPacket, ClientboundStatusResponsePacket, ServerboundPingPacket, ServerboundStatusRequestPacket};

pub mod login;
pub mod status;
pub mod clientbound;
pub mod serverbound;

pub enum ConnectionState {
    Handshake,
    Status,
    Login,
    Configuration,
    Play,
}

pub enum PacketDirection {
    Serverbound,
    Clientbound,
}

pub trait Packet: Sized + NetworkType + Debug {
    const STATE: ConnectionState;
    const DIRECTION: PacketDirection;
    const ID: i32;
}

packet_handlers! {
  Handshake {
    serverbound: [ServerboundHandshakePacket],
    clientbound: [],
  }

  Status {
    serverbound: [ServerboundStatusRequestPacket, ServerboundPingPacket],
    clientbound: [ClientboundStatusResponsePacket, ClientboundPongPacket],
  }

  Login {
    serverbound: [ServerboundLoginStartPacket, ServerboundLoginAcknowledgedPacket],
    clientbound: [ClientboundLoginSuccessPacket],
  }

  Configuration {
    serverbound: [
            ServerboundClientInformationPacket,
            ServerboundAcknowledgeFinishConfigurationPacket,
            ServerboundKnownPacksPacket,
            ServerboundAcceptCodeOfConductPacket,
            ServerboundConfigurationKeepAliveResponsePacket,
        ],
    clientbound: []
  }

  Play {
    serverbound: [],
    clientbound: [],
  }
}
