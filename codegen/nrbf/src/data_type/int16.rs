use nom::{combinator::map, number::complete::le_i16, IResult};

use super::impl_primitive;
use crate::{
  combinator::into_failure,
  error::{error_position, ErrorWithInput},
};

/// 2.1.1 `INT16`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Int16(pub i16);

impl Int16 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, ErrorWithInput<'_>> {
    map(le_i16, Self)(input)
      .map_err(into_failure)
      .map_err(|err| err.map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedInt16)))
  }
}

impl_primitive!(Int16, i16, visit_i16, deserialize_i16);
