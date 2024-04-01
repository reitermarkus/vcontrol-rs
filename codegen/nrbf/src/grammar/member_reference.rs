use nom::{
  branch::alt,
  combinator::{map, opt},
  IResult,
};

use crate::{
  grammar::{Classes, NullObject},
  record::{BinaryLibrary, BinaryObjectString, MemberPrimitiveTyped, MemberPrimitiveUnTyped, MemberReference},
  BinaryParser,
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
  pub fn parse(input: &'i [u8], parser: &mut BinaryParser<'i>) -> IResult<&'i [u8], Self> {
    alt((
      map(MemberPrimitiveTyped::parse, Self::MemberPrimitiveTyped),
      map(MemberReference::parse, Self::MemberReference),
      map(BinaryObjectString::parse, Self::BinaryObjectString),
      map(NullObject::parse, Self::NullObject),
      map(|input| Classes::parse(input, parser), Self::Classes),
    ))(input)
  }
}

/// 2.7 Binary Record Grammar - `memberReference`
#[derive(Debug, Clone, PartialEq)]
pub struct MemberReference2<'i> {
  pub member_reference: MemberReferenceInner<'i>,
}

impl<'i> MemberReference2<'i> {
  pub fn parse(input: &'i [u8], parser: &mut BinaryParser<'i>) -> IResult<&'i [u8], Self> {
    let (input, ()) = parser.parse_binary_library(input)?;

    let (input, member_reference) = MemberReferenceInner::parse(input, parser)?;

    Ok((input, Self { member_reference }))
  }
}
