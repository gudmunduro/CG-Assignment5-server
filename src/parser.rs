use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{map, map_parser, value},
    number::complete::{le_f32, le_u8},
    sequence::{preceded, tuple},
    IResult,
};

use crate::models::{GamePacket, StatusUpdate, Vector3};

pub fn parse_float(input: &[u8]) -> IResult<&[u8], f32> {
    map_parser(take(4u8), le_f32)(input)
}

pub fn parse_vector3(input: &[u8]) -> IResult<&[u8], Vector3> {
    map(
        tuple((parse_float, parse_float, parse_float)),
        |(x, y, z)| Vector3::new(x, y, z),
    )(input)
}

pub fn parse_register(input: &[u8]) -> IResult<&[u8], GamePacket> {
    value(GamePacket::Register, tag(&[0u8]))(input)
}

pub fn parse_status_update(input: &[u8]) -> IResult<&[u8], GamePacket> {
    map(
        preceded(tag(&[1u8]), tuple((le_u8, parse_vector3, parse_float, parse_float))),
        |(player_id, pos, rot, steering_angle)| GamePacket::StatusUpdate(StatusUpdate::new(player_id, pos, rot, steering_angle)),
    )(input)
}

pub fn parse_end_packet(input: &[u8]) -> IResult<&[u8], GamePacket> {
    map(preceded(tag(&[3u8]), le_u8), |player_id| GamePacket::End {
        player_id,
    })(input)
}

pub fn parse_drop_player(input: &[u8]) -> IResult<&[u8], GamePacket> {
    map(preceded(tag(&[4u8]), le_u8), |player_id| {
        GamePacket::DropPlayer { player_id }
    })(input)
}

pub fn parse_inform(input: &[u8]) -> IResult<&[u8], GamePacket> {
    map(preceded(tag(&[5u8]), le_u8), |player_id| {
        GamePacket::Inform { player_id }
    })(input)
}

pub fn parse_new_player(input: &[u8]) -> IResult<&[u8], GamePacket> {
    map(preceded(tag(&[6u8]), le_u8), |player_id| {
        GamePacket::NewPlayer { player_id }
    })(input)
}

pub fn parse_packet(packet: &[u8]) -> Result<GamePacket, Box<dyn std::error::Error + '_>> {
    let (_, packet) = alt((
        parse_status_update,
        parse_register,
        parse_end_packet,
        parse_drop_player,
        parse_inform,
        parse_new_player,
    ))(packet)?;

    Ok(packet)
}
