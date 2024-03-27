use nom::{
  branch::alt,
  combinator::{map, opt},
  IResult,
};

use crate::{
  grammar::{Classes, NullObject},
  record::{BinaryLibrary, BinaryObjectString, MemberPrimitiveTyped, MemberPrimitiveUnTyped, MemberReference},
};

#[derive(Debug, Clone, PartialEq)]
pub enum MemberReferenceInner<'i> {
  MemberPrimitiveUnTyped(MemberPrimitiveUnTyped),
  MemberPrimitiveTyped(MemberPrimitiveTyped),
  MemberReference(MemberReference),
  BinaryObjectString(BinaryObjectString<'i>),
  NullObject(NullObject),
  Classes(Classes<'i>),
}

impl<'i> MemberReferenceInner<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    alt((
      map(MemberPrimitiveTyped::parse, Self::MemberPrimitiveTyped),
      map(MemberReference::parse, Self::MemberReference),
      map(BinaryObjectString::parse, Self::BinaryObjectString),
      map(NullObject::parse, Self::NullObject),
      map(Classes::parse, Self::Classes),
    ))(input)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemberReference2<'i> {
  pub binary_library: Option<BinaryLibrary<'i>>,
  pub member_reference: MemberReferenceInner<'i>,
}

impl<'i> MemberReference2<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, binary_library) = opt(BinaryLibrary::parse)(input)?;
    let (input, member_reference) = MemberReferenceInner::parse(input)?;

    Ok((input, Self { binary_library, member_reference }))
  }
}
