use nom::{branch::alt, combinator::map, IResult, Parser, ToUsize};

use crate::{
  data_type::{Byte, Int32},
  record::RecordType,
  ClassInfo, MemberTypeInfo,
};

/// 2.3.2.1 `ClassWithMembersAndTypes`
#[derive(Debug, Clone, PartialEq)]
pub struct ClassWithMembersAndTypes<'i> {
  pub class_info: ClassInfo<'i>,
  pub member_type_info: MemberTypeInfo<'i>,
  pub library_id: Int32,
}

impl<'i> ClassWithMembersAndTypes<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, _) = RecordType::ClassWithMembersAndTypes.parse(input)?;

    let (input, class_info) = ClassInfo::parse(input)?;
    let (input, member_type_info) = MemberTypeInfo::parse(input, &class_info)?;
    let (input, library_id) = Int32::parse_positive(input)?;

    Ok((input, Self { class_info, member_type_info, library_id }))
  }
}
