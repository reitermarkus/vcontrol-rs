use nom::{combinator::map, number::complete::i8, IResult};

use super::impl_primitive;
use crate::combinator::into_failure;

/// 2.1.1 `INT8`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Int8(pub i8);

impl Int8 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(i8, Self)(input).map_err(into_failure)
  }
}

impl_primitive!(Int8, i8, visit_i8, deserialize_i8);
