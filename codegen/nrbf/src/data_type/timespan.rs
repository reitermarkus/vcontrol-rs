use nom::{combinator::map, number::complete::le_i64, IResult};

use crate::{
  combinator::into_failure,
  enumeration::PrimitiveType,
  error::{error_position, Error},
};

/// 2.1.1.4 `TimeSpan`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeSpan(pub i64);

impl TimeSpan {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, Error<'_>> {
    map(le_i64, Self)(input).map_err(into_failure).map_err(|err| {
      err.map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedPrimitive(PrimitiveType::TimeSpan)))
    })
  }
}

impl From<i64> for TimeSpan {
  #[inline]
  fn from(v: i64) -> Self {
    Self(v)
  }
}

impl From<TimeSpan> for i64 {
  #[inline]
  fn from(val: TimeSpan) -> Self {
    val.0
  }
}
