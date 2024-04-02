use nom::{combinator::map, number::complete::le_i64, IResult};

/// 2.1.1.5 `DateTime`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DateTime(pub i64);

impl DateTime {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(le_i64, Self)(input)
  }
}

impl From<i64> for DateTime {
  #[inline]
  fn from(v: i64) -> Self {
    Self(v)
  }
}

impl From<DateTime> for i64 {
  #[inline]
  fn from(val: DateTime) -> Self {
    val.0
  }
}
