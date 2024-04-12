use nom::{
  branch::alt,
  combinator::{map, map_opt},
  number::complete::{le_u16, le_u24, le_u32, u8},
  IResult,
};

use super::impl_primitive;
use crate::{
  combinator::into_failure,
  enumeration::PrimitiveType,
  error::{error_position, Error},
};

/// 2.1.1.1 `Char`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Char(pub char);

impl Char {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, Error<'_>> {
    map(
      alt((
        map_opt(u8, |n| char::from_u32(n as u32)),
        map_opt(le_u16, |n| char::from_u32(n as u32)),
        map_opt(le_u24, char::from_u32),
        map_opt(le_u32, char::from_u32),
      )),
      Self,
    )(input)
    .map_err(into_failure)
    .map_err(|err| {
      err.map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedPrimitive(PrimitiveType::Char)))
    })
  }
}

impl_primitive!(Char, char, visit_char, deserialize_char);
