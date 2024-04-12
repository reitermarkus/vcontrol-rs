use std::num::NonZeroU32;

use nom::IResult;

use crate::{
  combinator::library_id,
  common::{ClassInfo, MemberTypeInfo},
  error::{error_position, Error},
  record::RecordType,
};

/// 2.3.2.1 `ClassWithMembersAndTypes`
#[derive(Debug, Clone, PartialEq)]
pub struct ClassWithMembersAndTypes<'i> {
  pub class_info: ClassInfo<'i>,
  pub member_type_info: MemberTypeInfo<'i>,
  pub library_id: NonZeroU32,
}

impl<'i> ClassWithMembersAndTypes<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self, Error<'i>> {
    let (input, _) = RecordType::ClassWithMembersAndTypes.parse(input)?;

    let (input, class_info) =
      ClassInfo::parse(input).map_err(|err| err.map(|err| error_position!(err.input, ExpectedClassInfo)))?;
    let (input, member_type_info) = MemberTypeInfo::parse(input, &class_info)?;
    let (input, library_id) = library_id(input)?;

    Ok((input, Self { class_info, member_type_info, library_id }))
  }

  #[inline]
  pub fn class_info(&self) -> &ClassInfo<'i> {
    &self.class_info
  }

  #[inline]
  pub(crate) fn object_id(&self) -> NonZeroU32 {
    self.class_info().object_id()
  }

  #[inline]
  pub(crate) fn library_id(&self) -> NonZeroU32 {
    self.library_id
  }
}
