use nom::{combinator::map, number::complete::le_f64, IResult};

/// 2.1.1.2 `Double`
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
