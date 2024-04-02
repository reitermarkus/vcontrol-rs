use nom::IResult;

use crate::{
  binary_parser::Object,
  common::ClassInfo,
  data_type::Int32,
  grammar::MemberReferenceInner,
  record::{ClassWithMembers, ClassWithMembersAndTypes, SystemClassWithMembers, SystemClassWithMembersAndTypes},
  BinaryParser,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Class<'i> {
  ClassWithMembers(ClassWithMembers<'i>),
  ClassWithMembersAndTypes(ClassWithMembersAndTypes<'i>),
  SystemClassWithMembers(SystemClassWithMembers<'i>),
  SystemClassWithMembersAndTypes(SystemClassWithMembersAndTypes<'i>),
}

impl<'i> Class<'i> {
  pub fn class_info(&self) -> &ClassInfo<'i> {
    match self {
      Self::ClassWithMembers(class) => &class.class_info,
      Self::ClassWithMembersAndTypes(class) => &class.class_info,
      Self::SystemClassWithMembers(class) => &class.class_info,
      Self::SystemClassWithMembersAndTypes(class) => &class.class_info,
    }
  }
}

/// 2.7 Binary Record Grammar - `Classes`
#[derive(Debug, Clone, PartialEq)]
pub struct Classes<'i> {
  pub class_id: Int32,
  pub member_references: Vec<Object<'i>>,
}

impl<'i> Classes<'i> {
  pub fn parse(input: &'i [u8], parser: &mut BinaryParser<'i>) -> IResult<&'i [u8], Self> {
    let (input, ()) = parser.parse_binary_library(input)?;

    let (input, (class_id, member_references)) = parser.parse_class(input)?;

    Ok((input, Self { class_id, member_references }))
  }

  #[inline]
  pub(crate) fn object_id(&self) -> Int32 {
    self.class_id
  }
}
