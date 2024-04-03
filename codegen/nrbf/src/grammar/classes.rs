use nom::{
  branch::alt,
  combinator::{map, opt},
  multi::many0,
  IResult,
};

use crate::{
  common::MemberTypeInfo,
  data_type::Int32,
  grammar::MemberReference2,
  record::{
    BinaryArray, BinaryLibrary, ClassWithId, ClassWithMembers, ClassWithMembersAndTypes, SystemClassWithMembers,
    SystemClassWithMembersAndTypes,
  },
};

#[derive(Debug, Clone, PartialEq)]
pub enum Class<'i> {
  ClassWithId(ClassWithId),
  ClassWithMembers(ClassWithMembers<'i>),
  ClassWithMembersAndTypes(ClassWithMembersAndTypes<'i>),
  SystemClassWithMembers(SystemClassWithMembers<'i>),
  SystemClassWithMembersAndTypes(SystemClassWithMembersAndTypes<'i>),
}

impl<'i> Class<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    alt((
      map(ClassWithId::parse, Self::ClassWithId),
      map(ClassWithMembers::parse, Self::ClassWithMembers),
      map(ClassWithMembersAndTypes::parse, Self::ClassWithMembersAndTypes),
      map(SystemClassWithMembers::parse, Self::SystemClassWithMembers),
      map(SystemClassWithMembersAndTypes::parse, Self::SystemClassWithMembersAndTypes),
    ))(input)
  }

  #[inline]
  pub(crate) fn object_id(&self) -> Int32 {
    match self {
      Self::ClassWithId(class) => class.object_id(),
      Self::ClassWithMembers(class) => class.object_id(),
      Self::ClassWithMembersAndTypes(class) => class.object_id(),
      Self::SystemClassWithMembers(class) => class.object_id(),
      Self::SystemClassWithMembersAndTypes(class) => class.object_id(),
    }
  }
}

/// 2.7 Binary Record Grammar - `Classes`
#[derive(Debug, Clone, PartialEq)]
pub struct Classes<'i> {
  pub binary_library: Option<BinaryLibrary<'i>>,
  pub class: Class<'i>,
  pub member_references: Vec<MemberReference2<'i>>,
}

impl<'i> Classes<'i> {
  fn parse_member_references(
    mut input: &'i [u8],
    member_type_info: &MemberTypeInfo<'i>,
  ) -> IResult<&'i [u8], Vec<MemberReference2<'i>>> {
    let mut member_references = vec![];

    for (binary_type_enum, additional_info) in
      member_type_info.binary_type_enums.iter().zip(member_type_info.additional_infos.iter())
    {
      let member;
      (input, member) = BinaryArray::parse_member(input, *binary_type_enum, additional_info.as_ref())?;
      member_references.push(member);
    }

    Ok((input, member_references))
  }

  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, binary_library) = opt(BinaryLibrary::parse)(input)?;

    let (input, class) = Class::parse(input)?;

    let (input, member_references) = match class {
      Class::ClassWithId(ref _class) => many0(MemberReference2::parse)(input)?,
      Class::ClassWithMembers(ref _class) => many0(MemberReference2::parse)(input)?,
      Class::ClassWithMembersAndTypes(ref class) => Self::parse_member_references(input, &class.member_type_info)?,
      Class::SystemClassWithMembers(ref _class) => many0(MemberReference2::parse)(input)?,
      Class::SystemClassWithMembersAndTypes(ref class) => {
        Self::parse_member_references(input, &class.member_type_info)?
      },
    };

    Ok((input, Self { binary_library, class, member_references }))
  }

  #[inline]
  pub(crate) fn object_id(&self) -> Int32 {
    self.class.object_id()
  }
}
