use std::num::NonZeroU32;

use nom::IResult;

use crate::{common::ArrayInfo, error::Error, record::RecordType};

/// 2.4.3.4 `ArraySingleString`
#[derive(Debug, Clone, PartialEq)]
pub struct ArraySingleString {
  pub array_info: ArrayInfo,
}

impl ArraySingleString {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, Error<'_>> {
    let (input, _) = RecordType::ArraySingleString.parse(input)?;

    let (input, array_info) = ArrayInfo::parse(input)?;

    Ok((input, Self { array_info }))
  }

  #[inline]
  pub(crate) fn object_id(&self) -> NonZeroU32 {
    self.array_info.object_id()
  }
}
