use std::num::TryFromIntError;

use nom::{
  combinator::{map, verify},
  number::complete::le_i32,
  IResult,
};

use super::impl_primitive;

/// 2.1.1 `INT32`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Int32(pub i32);

impl Int32 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(le_i32, Self)(input)
  }

  pub fn parse_positive(input: &[u8]) -> IResult<&[u8], Self> {
    verify(Self::parse, |n| n.0 > 0)(input)
  }

  pub fn parse_positive_or_zero(input: &[u8]) -> IResult<&[u8], Self> {
    verify(Self::parse, |n| n.0 >= 0)(input)
  }
}

impl TryFrom<Int32> for usize {
  type Error = TryFromIntError;

  #[inline]
  fn try_from(val: Int32) -> Result<Self, Self::Error> {
    Self::try_from(val.0)
  }
}

impl_primitive!(Int32, i32, visit_i32, deserialize_i32);
