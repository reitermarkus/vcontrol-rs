use nom::{combinator::map, number::complete::le_f64, IResult};

use super::impl_primitive;
use crate::{
  combinator::into_failure,
  enumeration::PrimitiveType,
  error::{error_position, Error},
};

/// 2.1.1.2 `Double`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Double(pub f64);

impl Double {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, Error<'_>> {
    map(le_f64, Self)(input).map_err(into_failure).map_err(|err| {
      err.map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedPrimitive(PrimitiveType::Double)))
    })
  }
}

impl_primitive!(Double, f64, visit_f64, deserialize_f64);
