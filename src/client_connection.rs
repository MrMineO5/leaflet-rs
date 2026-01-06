use std::time::{Duration, SystemTime};
use crate::packet_reader::PacketReader;
use leaflet_network_buffer::McBuf;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use leaflet_protocol::{handle_configuration_serverbound, handle_handshake_serverbound, handle_login_serverbound, handle_play_serverbound, handle_status_serverbound, ConnectionState, Packet};
use leaflet_protocol::clientbound::configuration::configuration_keep_alive::ClientboundConfigurationKeepAlivePacket;
use leaflet_protocol::clientbound::play::keep_alive::ClientboundKeepAlivePacket;
use crate::handlers::HANDLERS;

const KEEP_ALIVE_INTERVAL: Duration = Duration::from_secs(1);

pub struct ClientConnection {
    connection: TcpStream,
    pub state: ConnectionState,
    packet_reader: PacketReader,
    packet_queue: Vec<McBuf>,
    last_keep_alive: SystemTime,
}

impl ClientConnection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            connection: stream,
            state: ConnectionState::Handshake,
            packet_reader: PacketReader::new(),
            packet_queue: Vec::new(),
            last_keep_alive: SystemTime::now(),
        }
    }

    pub async fn poll(&mut self) {
        let mut buf = [0u8; 1024];
        let read = self.connection.try_read(&mut buf);
        if let Ok(read) = read {
            self.packet_reader.append(&buf[..read]);
            self.process_incoming_packets();
        }

        if self.last_keep_alive.elapsed().unwrap_or_default() > KEEP_ALIVE_INTERVAL {
            self.last_keep_alive = SystemTime::now();
            match self.state {
                ConnectionState::Configuration => self.queue_packet(&ClientboundConfigurationKeepAlivePacket {
                    id: 0
                }),
                ConnectionState::Play => self.queue_packet(&ClientboundKeepAlivePacket {
                    id: 0
                }),
                _ => {}
            }
        }
        self.process_outgoing_packets().await;
    }

    pub fn queue_packet<T: Packet>(&mut self, packet: &T) {
        let mut buf = McBuf::new();
        buf.write_var_int(T::ID);
        buf.write_network_type(packet);
        self.packet_queue.push(buf);
    }

    fn process_incoming_packets(&mut self) {
        while let Some(mut packet) = self.packet_reader.read_packet() {
            match self.state {
                ConnectionState::Handshake => {
                    handle_handshake_serverbound(self, &mut packet, &HANDLERS).unwrap()
                }
                ConnectionState::Status => {
                    handle_status_serverbound(self, &mut packet, &HANDLERS).unwrap()
                }
                ConnectionState::Login => {
                    handle_login_serverbound(self, &mut packet, &HANDLERS).unwrap()
                }
                ConnectionState::Configuration => {
                    handle_configuration_serverbound(self, &mut packet, &HANDLERS).unwrap()
                }
                ConnectionState::Play => {
                    handle_play_serverbound(self, &mut packet, &HANDLERS).unwrap()
                }
            }
        }
    }

    async fn process_outgoing_packets(&mut self) {
        for packet in self.packet_queue.drain(..) {
            let len = packet.length();

            let mut rem = len;
            while (rem & !0x7F) != 0 {
                self.connection
                    .write_u8((rem & 0x7F) as u8 | 0x80)
                    .await
                    .unwrap();
                rem >>= 7;
            }
            self.connection.write_u8(rem as u8).await.unwrap();

            self.connection.write(packet.as_slice()).await.unwrap();
        }
        self.connection.flush().await.unwrap();
    }
}
