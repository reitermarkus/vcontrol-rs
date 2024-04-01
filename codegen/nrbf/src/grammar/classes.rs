use nom::{
  branch::alt,
  combinator::{map, opt},
  multi::many0,
  IResult,
};

use crate::{
  common::{ClassInfo, MemberTypeInfo},
  data_type::Int32,
  grammar::MemberReference2,
  record::{
    BinaryArray, BinaryLibrary, ClassWithId, ClassWithMembers, ClassWithMembersAndTypes, SystemClassWithMembers,
    SystemClassWithMembersAndTypes,
  },
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
  pub member_references: Vec<MemberReference2<'i>>,
}

impl<'i> Classes<'i> {
  pub fn parse_member_references(
    mut input: &'i [u8],
    member_type_info: &MemberTypeInfo<'i>,
    parser: &mut BinaryParser<'i>,
  ) -> IResult<&'i [u8], Vec<MemberReference2<'i>>> {
    let mut member_references = vec![];

    for (binary_type_enum, additional_info) in
      member_type_info.binary_type_enums.iter().zip(member_type_info.additional_infos.iter())
    {
      let member;
      (input, member) = BinaryArray::parse_member(input, *binary_type_enum, additional_info.as_ref(), parser)?;
      member_references.push(member);
    }

    Ok((input, member_references))
  }

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
