use nom::{
  combinator::{map, verify},
  number::complete::u8,
  IResult, Parser,
};

use super::impl_primitive;
use crate::combinator::into_failure;

/// 2.1.1 `BYTE`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Byte(pub u8);

impl Byte {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(u8, Self)(input).map_err(into_failure)
  }

  pub fn parse_positive(input: &[u8]) -> IResult<&[u8], Self> {
    verify(Self::parse, |n| n.0 > 0)(input).map_err(into_failure)
  }
}

impl_primitive!(Byte, u8, visit_u8, deserialize_u8);
