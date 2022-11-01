#[derive(Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn to_binary_data(&self) -> Vec<u8> {
        [self.x.to_le_bytes(), self.y.to_le_bytes(), self.z.to_le_bytes()].concat()
    }
}

#[derive(Clone)]
pub struct StatusUpdate {
    pub player_id: u8,
    pub position: Vector3,
    pub rotation: f32,
    pub steering_angle: f32
}

impl StatusUpdate {
    pub fn new(player_id: u8, position: Vector3, rotation: f32, steering_angle: f32) -> StatusUpdate {
        StatusUpdate { player_id, position, rotation, steering_angle }
    }

    pub fn to_binary_data(&self) -> Vec<u8> {
        [vec![1u8, self.player_id], self.position.to_binary_data(), self.rotation.to_le_bytes().to_vec(), self.steering_angle.to_le_bytes().to_vec()].concat()
    }
}

#[derive(Clone)]
pub enum GamePacket {
    Register,
    Inform { player_id: u8 },
    NewPlayer { player_id: u8 },
    LapComplete { player_id: u8 },
    StatusUpdate(StatusUpdate),
    Restart,
    DropPlayer { player_id: u8 },
    End { player_id: u8 },
}

impl GamePacket {
    pub fn to_binary_data(&self) -> Vec<u8> {
        use GamePacket::*;
        match self {
            Register => vec![0],
            Inform { player_id } => vec![5, *player_id],
            NewPlayer { player_id } => vec![6, *player_id],
            LapComplete { player_id } => vec![7, *player_id],
            Restart => vec![8u8],
            StatusUpdate(s) => s.to_binary_data(),
            DropPlayer { player_id } => vec![4, *player_id],
            End { player_id } => vec![3, *player_id]
        }
    }
}
