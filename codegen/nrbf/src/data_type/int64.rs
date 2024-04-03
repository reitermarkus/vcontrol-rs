use nom::{combinator::map, number::complete::le_i64, IResult};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize, Serializer};

/// 2.1.1 `INT64`
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Int64(pub i64);

impl Int64 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(le_i64, Self)(input)
  }
}

impl From<i64> for Int64 {
  #[inline]
  fn from(v: i64) -> Self {
    Self(v)
  }
}

impl From<Int64> for i64 {
  #[inline]
  fn from(val: Int64) -> Self {
    val.0
  }
}

#[cfg(feature = "serde")]
impl Serialize for Int64 {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_i64(self.0)
  }
}
