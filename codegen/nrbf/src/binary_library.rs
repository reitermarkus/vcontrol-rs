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

use super::LengthPrefixedString;

/// 2.6.2 `BinaryLibrary`
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryLibrary<'i> {
  pub library_id: i32,
  pub library_name: LengthPrefixedString<'i>,
}

impl<'i> BinaryLibrary<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, _) = tag([12])(input)?;

    let (input, library_id) = le_i32(input)?;
    let (input, library_name) = LengthPrefixedString::parse(input)?;

    Ok((input, Self { library_id, library_name }))
  }
}
