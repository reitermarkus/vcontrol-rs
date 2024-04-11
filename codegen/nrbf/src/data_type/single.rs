use nom::{combinator::map, number::complete::le_f32, IResult};

use super::impl_primitive;
use crate::combinator::into_failure;

/// 2.1.1.3 `Single`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Single(pub f32);

impl Single {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(le_f32, Self)(input).map_err(into_failure)
  }
}

impl_primitive!(Single, f32, visit_f32, deserialize_f32);
