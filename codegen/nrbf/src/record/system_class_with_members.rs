use nom::{IResult, Parser};

use crate::{
  common::ClassInfo,
  data_type::Int32,
  error::{error_position, ErrorWithInput},
  record::RecordType,
};

/// 2.3.2.4 `SystemClassWithMembers`
#[derive(Debug, Clone, PartialEq)]
pub struct SystemClassWithMembers<'i> {
  pub class_info: ClassInfo<'i>,
}

impl<'i> SystemClassWithMembers<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self, ErrorWithInput<'i>> {
    let (input, _) = RecordType::SystemClassWithMembers.parse(input).map_err(|err| {
      err.map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedSystemClassWithMembers))
    })?;

    let (input, class_info) =
      ClassInfo::parse(input).map_err(|err| err.map(|err| error_position!(err.input, ExpectedClassInfo)))?;

    Ok((input, Self { class_info }))
  }

  #[inline]
  pub fn class_info(&self) -> &ClassInfo<'i> {
    &self.class_info
  }

  #[inline]
  pub(crate) fn object_id(&self) -> Int32 {
    self.class_info.object_id()
  }
}
