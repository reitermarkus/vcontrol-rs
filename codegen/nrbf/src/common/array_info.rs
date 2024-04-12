use nom::{IResult, ToUsize};

use crate::{
  data_type::Int32,
  error::{ErrorWithInput},
};

/// 2.4.2.1 `ArrayInfo`
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayInfo {
  pub object_id: Int32,
  pub length: Int32,
}

impl ArrayInfo {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, ErrorWithInput<'_>> {
    let (input, object_id) = Int32::parse_positive(input)?;
    let (input, length) = Int32::parse_positive_or_zero(input)?;

    Ok((input, Self { object_id, length }))
  }

  #[inline]
  pub(crate) fn object_id(&self) -> Int32 {
    self.object_id
  }

  #[inline]
  pub(crate) fn len(&self) -> usize {
    (i32::from(self.length) as u32).to_usize()
  }
}
