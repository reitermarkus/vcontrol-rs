use nom::{
  combinator::{map, verify},
  number::complete::u8,
  IResult, Parser,
};

use super::impl_primitive;
use crate::{
  combinator::into_failure,
  error::{error_position, ErrorWithInput},
};

/// 2.1.1 `BYTE`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Byte(pub u8);

impl Byte {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, ErrorWithInput<'_>> {
    map(u8, Self)(input)
      .map_err(into_failure)
      .map_err(|err| err.map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedByte)))
  }

  pub fn parse_positive(input: &[u8]) -> IResult<&[u8], Self, ErrorWithInput<'_>> {
    verify(Self::parse, |n| n.0 > 0)(input)
      .map_err(into_failure)
      .map_err(|err| err.map(|err| error_position!(err.input, ExpectedByte)))
  }
}

impl_primitive!(Byte, u8, visit_u8, deserialize_u8);
