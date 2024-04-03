use std::num::TryFromIntError;

use nom::{
  combinator::{map, verify},
  number::complete::le_i32,
  IResult,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize, Serializer};

/// 2.1.1 `INT32`
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Int32(pub i32);

impl Int32 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(le_i32, Self)(input)
  }

  pub fn parse_positive(input: &[u8]) -> IResult<&[u8], Self> {
    verify(Self::parse, |n| n.0 > 0)(input)
  }

  pub fn parse_positive_or_zero(input: &[u8]) -> IResult<&[u8], Self> {
    verify(Self::parse, |n| n.0 >= 0)(input)
  }
}

impl From<i32> for Int32 {
  #[inline]
  fn from(v: i32) -> Self {
    Self(v)
  }
}

impl From<Int32> for i32 {
  #[inline]
  fn from(val: Int32) -> Self {
    val.0
  }
}

impl TryFrom<Int32> for usize {
  type Error = TryFromIntError;

  #[inline]
  fn try_from(val: Int32) -> Result<Self, Self::Error> {
    Self::try_from(val.0)
  }
}

#[cfg(feature = "serde")]
impl Serialize for Int32 {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_i32(self.0)
  }
}
