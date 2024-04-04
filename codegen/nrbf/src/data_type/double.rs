use nom::{combinator::map, number::complete::le_f64, IResult};

use super::impl_primitive;

/// 2.1.1.2 `Double`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Double(pub f64);

impl Double {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(le_f64, Self)(input)
  }
}

impl_primitive!(Double, f64, visit_f64, deserialize_f64);
