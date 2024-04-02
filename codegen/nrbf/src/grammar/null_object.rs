use nom::{branch::alt, combinator::map, IResult};

use crate::record::{ObjectNull, ObjectNullMultiple, ObjectNullMultiple256};

/// 2.7 Binary Record Grammar - `nullObject`
#[derive(Debug, Clone, PartialEq)]
pub enum NullObject {
  ObjectNull(ObjectNull),
  ObjectNullMultiple(ObjectNullMultiple),
  ObjectNullMultiple256(ObjectNullMultiple256),
}

impl NullObject {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    alt((
      map(ObjectNull::parse, Self::ObjectNull),
      map(ObjectNullMultiple::parse, Self::ObjectNullMultiple),
      map(ObjectNullMultiple256::parse, Self::ObjectNullMultiple256),
    ))(input)
  }

  #[inline]
  pub(crate) fn null_count(&self) -> usize {
    match self {
      Self::ObjectNull(object_null) => object_null.null_count(),
      Self::ObjectNullMultiple(object_null) => object_null.null_count(),
      Self::ObjectNullMultiple256(object_null) => object_null.null_count(),
    }
  }
}
