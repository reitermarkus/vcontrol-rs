use nom::{combinator::map, number::complete::le_u64, IResult};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize, Serializer};

/// 2.1.1 `UINT64`
#[cfg_attr(feature = "serde", derive(Deserialize))]
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

#[cfg(feature = "serde")]
impl Serialize for UInt64 {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_u64(self.0)
  }
}
