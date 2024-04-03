use nom::{combinator::map, number::complete::le_f64, IResult};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize, Serializer};

/// 2.1.1.2 `Double`
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Double(pub f64);

impl Double {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(le_f64, Self)(input)
  }
}

impl From<f64> for Double {
  #[inline]
  fn from(v: f64) -> Self {
    Self(v)
  }
}

impl From<Double> for f64 {
  #[inline]
  fn from(val: Double) -> Self {
    val.0
  }
}

#[cfg(feature = "serde")]
impl Serialize for Double {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_f64(self.0)
  }
}
