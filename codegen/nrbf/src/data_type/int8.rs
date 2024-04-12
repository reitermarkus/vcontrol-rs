use nom::{combinator::map, number::complete::i8, IResult};

use super::impl_primitive;
use crate::{
  combinator::into_failure,
  enumeration::PrimitiveType,
  error::{error_position, Error},
};

/// 2.1.1 `INT8`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Int8(pub i8);

impl Int8 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, Error<'_>> {
    map(i8, Self)(input).map_err(into_failure).map_err(|err| {
      err.map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedPrimitive(PrimitiveType::SByte)))
    })
  }
}

impl_primitive!(Int8, i8, visit_i8, deserialize_i8);
