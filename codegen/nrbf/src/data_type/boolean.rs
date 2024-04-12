use nom::{combinator::map_res, number::complete::u8, IResult};

use super::impl_primitive;
use crate::{
  combinator::into_failure,
  enumeration::PrimitiveType,
  error::{error_position, Error},
};

/// 2.1.1 `BOOLEAN`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Boolean(pub bool);

impl Boolean {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, Error<'_>> {
    map_res(u8, |byte| {
      Ok(Self(match byte {
        0 => false,
        1 => true,
        _ => return Err(()),
      }))
    })(input)
    .map_err(into_failure)
    .map_err(|err| {
      err.map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedPrimitive(PrimitiveType::Boolean)))
    })
  }
}

impl_primitive!(Boolean, bool, visit_bool, deserialize_bool);
