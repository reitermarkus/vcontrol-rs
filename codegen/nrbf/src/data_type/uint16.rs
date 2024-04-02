use nom::{combinator::map, number::complete::le_u16, IResult};

/// 2.1.1 `UINT16`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UInt16(pub u16);

impl UInt16 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(le_u16, Self)(input)
  }
}

impl From<u16> for UInt16 {
  #[inline]
  fn from(v: u16) -> Self {
    Self(v)
  }
}

impl From<UInt16> for u16 {
  #[inline]
  fn from(val: UInt16) -> Self {
    val.0
  }
}
