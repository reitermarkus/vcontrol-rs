use std::num::NonZeroU32;

use nom::IResult;

use crate::{
  combinator::{self, object_id},
  error::{error_position, Error},
  record::RecordType,
};

/// 2.3.2.5 `ClassWithId`
#[derive(Debug, Clone, PartialEq)]
pub struct ClassWithId {
  object_id: NonZeroU32,
  metadata_id: NonZeroU32,
}

impl ClassWithId {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, Error<'_>> {
    let (input, _) = RecordType::ClassWithId.parse(input)?;

    let (input, object_id) = object_id(input)?;
    let err_input = input;
    let (input, metadata_id) = combinator::object_id(input)?;

    if metadata_id == object_id {
      return Err(nom::Err::Failure(error_position!(err_input, InvalidMetadataId)))
    }

    Ok((input, Self { object_id, metadata_id }))
  }

  #[inline]
  pub(crate) fn object_id(&self) -> NonZeroU32 {
    self.object_id
  }

  #[inline]
  pub(crate) fn metadata_id(&self) -> NonZeroU32 {
    self.metadata_id
  }
}
