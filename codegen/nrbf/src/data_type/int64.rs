use nom::{combinator::map, number::complete::le_i64, IResult};

use super::impl_primitive;
use crate::combinator::into_failure;

/// 2.1.1 `INT64`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Int64(pub i64);

impl Int64 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(le_i64, Self)(input).map_err(into_failure)
  }
}

impl_primitive!(Int64, i64, visit_i64, deserialize_i64);
