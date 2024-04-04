use nom::{combinator::map, number::complete::le_i16, IResult};

use super::impl_primitive;

/// 2.1.1 `INT16`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Int16(pub i16);

impl Int16 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(le_i16, Self)(input)
  }
}

impl_primitive!(Int16, i16, visit_i16, deserialize_i16);
