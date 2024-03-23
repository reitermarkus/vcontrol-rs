use nom::{
  branch::alt,
  bytes::complete::{tag, take},
  combinator::{cond, map, map_opt, map_res, opt, value, verify},
  complete::bool,
  multi::{many0, many_m_n},
  number::complete::{i8, le_f32, le_f64, le_i16, le_i32, le_i64, le_u16, le_u32, le_u64, u8},
  sequence::{preceded, terminated},
  IResult,
};

/// 2.6.1 `SerializationHeaderRecord`
#[derive(Debug, Clone, PartialEq)]
pub struct SerializationHeader {
  pub root_id: i32,
  pub header_id: i32,
  pub major_version: i32,
  pub minor_version: i32,
}

impl SerializationHeader {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, _) = tag([0])(input)?;

    let (input, root_id) = le_i32(input)?;
    let (input, header_id) = le_i32(input)?;
    let (input, major_version) = le_i32(input)?;
    let (input, minor_version) = le_i32(input)?;

    Ok((input, Self { root_id, header_id, major_version, minor_version }))
  }
}
