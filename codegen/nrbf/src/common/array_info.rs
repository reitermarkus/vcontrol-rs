use std::num::NonZeroU32;

use nom::IResult;

use crate::{
  combinator::{length, object_id},
  error::Error,
};

/// 2.4.2.1 `ArrayInfo`
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayInfo {
  object_id: NonZeroU32,
  length: usize,
}

impl ArrayInfo {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, Error<'_>> {
    let (input, object_id) = object_id(input)?;
    let (input, length) = length(input)?;

    Ok((input, Self { object_id, length }))
  }

  #[inline]
  pub(crate) fn object_id(&self) -> NonZeroU32 {
    self.object_id
  }

  #[inline]
  pub(crate) fn len(&self) -> usize {
    self.length
  }
}
