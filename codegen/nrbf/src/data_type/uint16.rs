use nom::{combinator::map, number::complete::le_u16, IResult};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize, Serializer};

/// 2.1.1 `UINT16`
#[cfg_attr(feature = "serde", derive(Deserialize))]
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

#[cfg(feature = "serde")]
impl Serialize for UInt16 {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_u16(self.0)
  }
}
