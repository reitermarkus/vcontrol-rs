use nom::{combinator::map, number::complete::le_i64, IResult};

/// 2.1.1.4 `TimeSpan`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeSpan(pub i64);

impl TimeSpan {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(le_i64, Self)(input)
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
