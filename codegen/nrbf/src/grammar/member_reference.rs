use nom::{
  branch::alt,
  combinator::{map},
  IResult,
};

use crate::{
  grammar::{Classes, NullObject},
  record::{self, BinaryObjectString, MemberPrimitiveTyped, MemberPrimitiveUnTyped},
  BinaryParser,
};

#[derive(Debug, Clone, PartialEq)]
pub enum MemberReferenceInner<'i> {
  MemberPrimitiveUnTyped(MemberPrimitiveUnTyped),
  MemberPrimitiveTyped(MemberPrimitiveTyped),
  MemberReference(record::MemberReference),
  BinaryObjectString(BinaryObjectString<'i>),
  NullObject(NullObject),
  Classes(Classes<'i>),
}

impl<'i> MemberReferenceInner<'i> {
  pub fn parse(input: &'i [u8], parser: &mut BinaryParser<'i>) -> IResult<&'i [u8], Self> {
    alt((
      map(MemberPrimitiveTyped::parse, Self::MemberPrimitiveTyped),
      map(record::MemberReference::parse, Self::MemberReference),
      map(BinaryObjectString::parse, Self::BinaryObjectString),
      map(NullObject::parse, Self::NullObject),
      map(|input| Classes::parse(input, parser), Self::Classes),
    ))(input)
  }
}
