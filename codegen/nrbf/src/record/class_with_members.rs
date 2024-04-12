use std::num::NonZeroU32;

use nom::IResult;

use crate::{
  combinator::library_id,
  common::ClassInfo,
  error::{error_position, Error},
  record::RecordType,
};

/// 2.3.2.2 `ClassWithMembers`
#[derive(Debug, Clone, PartialEq)]
pub struct ClassWithMembers<'i> {
  pub class_info: ClassInfo<'i>,
  pub library_id: NonZeroU32,
}

impl<'i> ClassWithMembers<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self, Error<'i>> {
    let (input, _) = RecordType::ClassWithMembers.parse(input)?;

    let (input, class_info) =
      ClassInfo::parse(input).map_err(|err| err.map(|err| error_position!(err.input, ExpectedClassInfo)))?;
    let (input, library_id) = library_id(input)?;

    Ok((input, Self { class_info, library_id }))
  }

  #[inline]
  pub fn class_info(&self) -> &ClassInfo<'i> {
    &self.class_info
  }

  #[inline]
  pub(crate) fn object_id(&self) -> NonZeroU32 {
    self.class_info.object_id()
  }

  #[inline]
  pub(crate) fn library_id(&self) -> NonZeroU32 {
    self.library_id
  }
}
