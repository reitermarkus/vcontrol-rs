use nom::{combinator::map, number::complete::le_u64, IResult};

/// 2.1.1 `UINT64`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UInt64(pub u64);

impl UInt64 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(le_u64, Self)(input)
  }
}

impl From<u64> for UInt64 {
  #[inline]
  fn from(v: u64) -> Self {
    Self(v)
  }
}

impl From<UInt64> for u64 {
  #[inline]
  fn from(val: UInt64) -> Self {
    val.0
  }
}
