use std::net::SocketAddr;

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn binary_data(&self) -> Vec<u8> {
        [self.x.to_le_bytes(), self.y.to_le_bytes(), self.z.to_le_bytes()].concat()
    }
}

pub struct StatusUpdate {
    pub position: Vector3,
    pub rotation: f32
}

impl StatusUpdate {
    pub fn new(position: Vector3, rotation: f32) -> StatusUpdate {
        StatusUpdate { position, rotation }
    }

    pub fn binary_data(&self) -> Vec<u8> {
        [self.position.binary_data(), self.rotation.to_le_bytes().to_vec()].concat()
    }
}

pub struct RegisterPacket {
    pub player_id: u8
}

impl RegisterPacket {
    pub fn new(player_id: u8) -> RegisterPacket {
        RegisterPacket { player_id }
    }

    pub fn binary_data(&self) -> Vec<u8> {
        [0u8, self.player_id].to_vec()
    }
}

pub enum GamePacket {
    Register(RegisterPacket),
    StatusUpdate(StatusUpdate),
}

pub struct PlayerConnection {
    pub player_id: u8,
    pub address: SocketAddr
}

impl PlayerConnection {
    pub fn new(player_id: u8, address: SocketAddr) -> PlayerConnection {
        PlayerConnection { player_id, address }
    }
}