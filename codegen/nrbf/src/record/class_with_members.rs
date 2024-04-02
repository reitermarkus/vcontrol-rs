use nom::{IResult, Parser};

use crate::{common::ClassInfo, data_type::Int32, record::RecordType};

/// 2.3.2.2 `ClassWithMembers`
#[derive(Debug, Clone, PartialEq)]
pub struct ClassWithMembers<'i> {
  pub class_info: ClassInfo<'i>,
  pub library_id: Int32,
}

impl<'i> ClassWithMembers<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, _) = RecordType::ClassWithMembers.parse(input)?;

    let (input, class_info) = ClassInfo::parse(input)?;
    let (input, library_id) = Int32::parse_positive(input)?;

    Ok((input, Self { class_info, library_id }))
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
