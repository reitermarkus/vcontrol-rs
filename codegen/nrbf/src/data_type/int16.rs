use nom::{combinator::map, number::complete::le_i16, IResult};

/// 2.1.1 `INT16`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Int16(pub i16);

impl Int16 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(le_i16, Self)(input)
  }
}

impl From<i16> for Int16 {
  #[inline]
  fn from(v: i16) -> Self {
    Self(v)
  }
}

impl From<Int16> for i16 {
  #[inline]
  fn from(val: Int16) -> Self {
    val.0
  }
}
