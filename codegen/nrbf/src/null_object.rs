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

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct ObjectNull;

impl ObjectNull {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, _) = tag([10])(input)?;

    Ok((input, Self))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectNullMultiple {
  pub null_count: i32,
}

impl ObjectNullMultiple {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, _) = tag([14])(input)?;

    let (input, null_count) = le_i32(input)?;

    Ok((input, Self { null_count }))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectNullMultiple256 {
  null_count: u8,
}

impl ObjectNullMultiple256 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, _) = tag([13])(input)?;

    let (input, null_count) = u8(input)?;

    Ok((input, Self { null_count }))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NullObject {
  ObjectNull(ObjectNull),
  ObjectNullMultiple(ObjectNullMultiple),
  ObjectNullMultiple256(ObjectNullMultiple256),
}

impl NullObject {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    alt((
      map(ObjectNull::parse, Self::ObjectNull),
      map(ObjectNullMultiple::parse, Self::ObjectNullMultiple),
      map(ObjectNullMultiple256::parse, Self::ObjectNullMultiple256),
    ))(input)
  }
}