use nom::{combinator::map, number::complete::u8, IResult};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize, Serializer};

/// 2.1.1 `BYTE`
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Byte(pub u8);

impl Byte {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(u8, Self)(input)
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

#[cfg(feature = "serde")]
impl Serialize for Byte {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_u8(self.0)
  }
}
