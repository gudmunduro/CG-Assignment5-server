use nom::{IResult, branch::alt, sequence::{preceded, tuple}, bytes::complete::{tag, take}, number::complete::{le_f32, le_u8}};

use crate::models::{StatusUpdate, GamePacket, Vector3, RegisterPacket};

pub fn parse_float(input: &[u8]) -> IResult<&[u8], f32> {
    let (input, value) = take(4u8)(input)?;
    let (_, value) = le_f32(value)?;

    Ok((input, value))
}

pub fn parse_vector3(input: &[u8]) -> IResult<&[u8], Vector3> {
    let (input, (x, y, z)) = tuple((parse_float, parse_float, parse_float))(input)?;

    Ok((input, Vector3::new(x, y, z)))
}

pub fn parse_register(input: &[u8]) -> IResult<&[u8], GamePacket> {
    let (input, player_id) = preceded(tag(&[0u8]), le_u8)(input)?;

    Ok((input, GamePacket::Register(RegisterPacket::new(player_id))))
}

pub fn parse_status_update(input: &[u8]) -> IResult<&[u8], GamePacket> {
    let (input, (pos, rot)) = preceded(tag(&[1u8]), tuple((parse_vector3, parse_float)))(input)?;

    Ok((input, GamePacket::StatusUpdate(StatusUpdate::new(pos, rot))))
}

pub fn parse_packet(packet: &[u8]) -> Result<GamePacket, Box<dyn std::error::Error + '_>> {
    let (_, packet) = alt((parse_status_update,))(packet)?;

    Ok(packet)
}