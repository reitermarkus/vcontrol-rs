use std::num::NonZeroU32;

use nom::IResult;

use crate::{
  common::ClassInfo,
  error::{error_position, Error},
  record::RecordType,
};

/// 2.3.2.4 `SystemClassWithMembers`
#[derive(Debug, Clone, PartialEq)]
pub struct SystemClassWithMembers<'i> {
  pub class_info: ClassInfo<'i>,
}

impl<'i> SystemClassWithMembers<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self, Error<'i>> {
    let (input, _) = RecordType::SystemClassWithMembers.parse(input)?;

    let (input, class_info) =
      ClassInfo::parse(input).map_err(|err| err.map(|err| error_position!(err.input, ExpectedClassInfo)))?;

    Ok((input, Self { class_info }))
  }

  #[inline]
  pub fn class_info(&self) -> &ClassInfo<'i> {
    &self.class_info
  }

  #[inline]
  pub(crate) fn object_id(&self) -> NonZeroU32 {
    self.class_info.object_id()
  }
}
