use nom::{combinator::map, IResult};

use crate::{
  combinator::into_failure,
  data_type::Int64,
  enumeration::PrimitiveType,
  error::{error_position, Error},
};

/// 2.1.1.5 `DateTime`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DateTime(pub Int64);

impl DateTime {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, Error<'_>> {
    map(Int64::parse, Self)(input)
      .map_err(into_failure)
      .map_err(|err| err.map(|err| error_position!(err.input, ExpectedPrimitive(PrimitiveType::DateTime))))
  }
}

impl From<i64> for DateTime {
  #[inline]
  fn from(v: i64) -> Self {
    Self(v.into())
  }
}

impl From<DateTime> for i64 {
  #[inline]
  fn from(val: DateTime) -> Self {
    Self::from(val.0)
  }
}
