use nom::{combinator::map, number::complete::i8, IResult};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize, Serializer};

/// 2.1.1 `INT8`
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Int8(pub i8);

impl Int8 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(i8, Self)(input)
  }
}

impl From<i8> for Int8 {
  #[inline]
  fn from(v: i8) -> Self {
    Self(v)
  }
}

impl From<Int8> for i8 {
  #[inline]
  fn from(val: Int8) -> Self {
    val.0
  }
}

#[cfg(feature = "serde")]
impl Serialize for Int8 {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_i8(self.0)
  }
}
