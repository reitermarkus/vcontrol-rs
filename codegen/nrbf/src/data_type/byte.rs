use nom::{
  combinator::{map, verify},
  number::complete::u8,
  IResult,
};

/// 2.1.1 `BYTE`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Byte(pub u8);

impl Byte {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(u8, Self)(input)
  }

  pub fn parse_positive(input: &[u8]) -> IResult<&[u8], Self> {
    verify(Self::parse, |n| n.0 > 0)(input)
  }
}

impl From<u8> for Byte {
  #[inline]
  fn from(v: u8) -> Self {
    Self(v)
  }
}

impl From<Byte> for u8 {
  #[inline]
  fn from(val: Byte) -> Self {
    val.0
  }
}

impl From<Byte> for i32 {
  #[inline]
  fn from(val: Byte) -> Self {
    Self::from(val.0)
  }
}
