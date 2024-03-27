use nom::{branch::alt, combinator::map, IResult, Parser, ToUsize};

use crate::{
  data_type::{Byte, Int32},
  record::RecordType,
  ClassInfo, MemberTypeInfo,
};

/// 2.3.2.3 `SystemClassWithMembersAndTypes`
#[derive(Debug, Clone, PartialEq)]
pub struct SystemClassWithMembersAndTypes<'i> {
  pub class_info: ClassInfo<'i>,
  pub member_type_info: MemberTypeInfo<'i>,
}

impl<'i> SystemClassWithMembersAndTypes<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, _) = RecordType::SystemClassWithMembersAndTypes.parse(input)?;

    let (input, class_info) = ClassInfo::parse(input)?;
    let (input, member_type_info) = MemberTypeInfo::parse(input, &class_info)?;

    Ok((input, Self { class_info, member_type_info }))
  }
}
