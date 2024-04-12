use nom::{combinator::map, number::complete::le_u64, IResult};

use super::impl_primitive;
use crate::{
  combinator::into_failure,
  error::{error_position, ErrorWithInput},
};

/// 2.1.1 `UINT64`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UInt64(pub u64);

impl UInt64 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, ErrorWithInput<'_>> {
    map(le_u64, Self)(input)
      .map_err(into_failure)
      .map_err(|err| err.map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedUInt64)))
  }
}

impl_primitive!(UInt64, u64, visit_u64, deserialize_u64);
