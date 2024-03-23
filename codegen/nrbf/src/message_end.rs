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

/// 2.6.3 `MessageEnd`
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct MessageEnd;

impl MessageEnd {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, _) = tag([11])(input)?;

    Ok((input, Self))
  }
}
