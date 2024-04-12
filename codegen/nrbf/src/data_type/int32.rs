use std::num::TryFromIntError;

use nom::{combinator::map, number::complete::le_i32, IResult};

use super::impl_primitive;
use crate::{
  combinator::into_failure,
  enumeration::PrimitiveType,
  error::{error_position, Error},
};

/// 2.1.1 `INT32`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Int32(pub i32);

impl Int32 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, Error<'_>> {
    map(le_i32, Self)(input).map_err(into_failure).map_err(|err| {
      err.map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedPrimitive(PrimitiveType::Int32)))
    })
  }
}

impl TryFrom<Int32> for usize {
  type Error = TryFromIntError;

  #[inline]
  fn try_from(val: Int32) -> Result<Self, Self::Error> {
    Self::try_from(val.0)
  }
}

impl_primitive!(Int32, i32, visit_i32, deserialize_i32);
