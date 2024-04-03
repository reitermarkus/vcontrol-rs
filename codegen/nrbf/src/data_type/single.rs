use nom::{combinator::map, number::complete::le_f32, IResult};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize, Serializer};

/// 2.1.1.3 `Single`
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Single(pub f32);

impl Single {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(le_f32, Self)(input)
  }
}

impl From<f32> for Single {
  #[inline]
  fn from(v: f32) -> Self {
    Self(v)
  }
}

impl From<Single> for f32 {
  #[inline]
  fn from(val: Single) -> Self {
    val.0
  }
}

#[cfg(feature = "serde")]
impl Serialize for Single {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_f32(self.0)
  }
}
