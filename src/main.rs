use std::{
    io,
    net::{SocketAddr, UdpSocket},
};

use clap::Parser;
use simplelog::TermLogger;

use crate::{
    models::{GamePacket, PlayerConnection},
    parser::parse_packet,
};

pub mod models;
pub mod parser;

#[derive(Parser, Debug)]
#[command(about = "Server for assignment 5 game", long_about = None)]
struct Args {
    #[clap(value_parser, default_value = "0.0.0.0")]
    ip: String,
    #[clap(value_parser, default_value_t = 5899)]
    port: u32,
}


fn main() {
    let args = Args::parse();
    init_logger();

    let socket = UdpSocket::bind(format!("{}:{}", &args.ip, args.port)).expect("Failed to bind socket");
    socket.set_nonblocking(true).unwrap();

    log::info!("Server listening on {}:{}", &args.ip, args.port);

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

        match handle_packet(packet, &socket, &src, &mut player_connections) {
            Ok(()) => (),
            Err(e) => {
                log::error!("Failed to handle packet {e}");
            }
        }
    }
}

fn handle_packet(
    packet: GamePacket,
    socket: &UdpSocket,
    src: &SocketAddr,
    player_connections: &mut Vec<PlayerConnection>,
) -> anyhow::Result<()> {
    use GamePacket::*;
    match packet {
        Register => {
            let last_player_id = player_connections.last().map(|p| p.player_id).unwrap_or(0);
            let player_id = last_player_id + 1;
            player_connections.push(PlayerConnection::new(player_id, *src));
            log::info!("Registered new player with id {player_id} and ip address {src}");

            // Inform the player of their player id
            try_send_packet(socket, src, &GamePacket::Inform { player_id });

            // Inform all other players that a new player joined, and inform the new player of all other players that have joined
            for player in player_connections
                .iter()
                .filter(|p| p.player_id != player_id)
            {
                try_send_packet(
                    socket,
                    src,
                    &GamePacket::NewPlayer {
                        player_id: player.player_id,
                    },
                );
                try_send_packet(
                    socket,
                    &player.address,
                    &GamePacket::NewPlayer { player_id },
                );
            }
        }
        StatusUpdate(status) => {
            for player in player_connections.iter().filter(|p| p.address != *src) {
                try_send_packet(
                    socket,
                    &player.address,
                    &GamePacket::StatusUpdate(status.clone()),
                );
            }
        }
        End { player_id } => {
            player_connections.retain(|p| p.player_id != player_id);

            for player in player_connections
                .iter()
                .filter(|p| p.player_id != player_id)
            {
                let drop_packet = GamePacket::DropPlayer { player_id };
                try_send_packet(socket, &player.address, &drop_packet);
            }
            log::info!("Ended connection with player {player_id} at {src}");
            log::info!("Player count is now {}", player_connections.len());
        }
        // This should never be sent to the server
        DropPlayer { .. } | Inform { .. } | NewPlayer { .. } => (),
    }

    Ok(())
}

fn try_send_packet(socket: &UdpSocket, addr: &SocketAddr, packet: &GamePacket) {
    match socket.send_to(&packet.binary_data(), addr) {
        Ok(_) => (),
        Err(e) => {
            log::error!("Failed to send packet to {}. Error: {e}", &addr);
        }
    }
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
