use nom::{combinator::map, number::complete::le_u32, IResult};

/// 2.1.1 `UINT32`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UInt32(pub u32);

impl UInt32 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(le_u32, Self)(input)
  }
}

impl From<u32> for UInt32 {
  #[inline]
  fn from(v: u32) -> Self {
    Self(v)
  }
}

impl From<UInt32> for u32 {
  #[inline]
  fn from(val: UInt32) -> Self {
    val.0
  }
}
