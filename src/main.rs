use std::{io, net::{UdpSocket, SocketAddr}};

use simplelog::TermLogger;

use crate::{parser::parse_packet, models::{GamePacket, RegisterPacket, PlayerConnection}};

pub mod models;
pub mod parser;

const PORT: u32 = 5899;

fn main() {
    init_logger();

    let socket = UdpSocket::bind(format!("0.0.0.0:{PORT}")).expect("Failed to bind socket");
    socket.set_nonblocking(true).unwrap();

    log::info!("Server listening on {PORT}");

    let mut player_connections = Vec::new();
    let mut buffer = [0u8; 3000];
    loop {
        let (count, src) = match socket.recv_from(&mut buffer) {
            Ok(res) => res,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
            Err(e) => {
                log::error!("Error occurred while trying to read from socket {e}");
                continue;
            }
        };

        let packet = match parse_packet(&buffer[0..count]) {
            Ok(p) => p,
            Err(e) => {
                log::error!("Invalid packet received. Parse error {e}");
                continue;
            }
        };

        log::info!("Received {count} bytes from {src}");

        match handle_packet(packet, &socket, &src, &mut player_connections) {
            Ok(()) => (),
            Err(e) => {
                log::error!("Failed to handle packet {e}");
            }
        }
    }
}

fn handle_packet(packet: GamePacket, socket: &UdpSocket, src: &SocketAddr, player_connections: &mut Vec<PlayerConnection>) -> anyhow::Result<()> {
    use GamePacket::*;
    match packet {
        Register(RegisterPacket { player_id }) => {
            player_connections.push(PlayerConnection::new(player_id, *src));
            log::info!("Registered new player with id {player_id} and ip address {src}");
        }
        StatusUpdate(status) => {
            let statsu_data = status.binary_data();
            for player in player_connections.iter().filter(|p| p.address != *src) {
                match socket.send_to(&statsu_data, player.address) {
                    Ok(_) => (),
                    Err(e) => {
                        log::error!("Failed to send packet to {}. Error: {e}", &player.address);
                    }
                }
            }
            log::info!("Sent status update to all players");
        }
    }

    Ok(())
}

fn init_logger() {
    TermLogger::init(
        simplelog::LevelFilter::Info,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .expect("Failed to init logger");
}
