//! 2.1.2 Enumerations

use nom::{
  branch::alt, bytes::complete::tag, combinator::value, error::ParseError, Compare, IResult, InputLength, InputTake,
  Parser,
};

/// 2.1.2.1 `RecordTypeEnumeration`
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum RecordType {
  SerializedStreamHeader         = 0,
  ClassWithId                    = 1,
  SystemClassWithMembers         = 2,
  ClassWithMembers               = 3,
  SystemClassWithMembersAndTypes = 4,
  ClassWithMembersAndTypes       = 5,
  BinaryObjectString             = 6,
  BinaryArray                    = 7,
  MemberPrimitiveTyped           = 8,
  MemberReference                = 9,
  ObjectNull                     = 10,
  MessageEnd                     = 11,
  BinaryLibrary                  = 12,
  ObjectNullMultiple256          = 13,
  ObjectNullMultiple             = 14,
  ArraySinglePrimitive           = 15,
  ArraySingleObject              = 16,
  ArraySingleString              = 17,
  MethodCall                     = 21,
  MethodReturn                   = 22,
}

impl<I, E> Parser<I, Self, E> for RecordType
where
  I: InputTake + Compare<[u8; 1]>,
  E: ParseError<I>,
{
  fn parse(&mut self, input: I) -> IResult<I, Self, E> {
    value(*self, tag([*self as u8]))(input)
  }
}

/// 2.1.2.2 `BinaryTypeEnumeration`
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum BinaryType {
  Primitive      = 0,
  String         = 1,
  Object         = 2,
  SystemClass    = 3,
  Class          = 4,
  ObjectArray    = 5,
  StringArray    = 6,
  PrimitiveArray = 7,
}

impl BinaryType {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    alt((
      Self::Primitive,
      Self::String,
      Self::Object,
      Self::SystemClass,
      Self::Class,
      Self::ObjectArray,
      Self::StringArray,
      Self::PrimitiveArray,
    ))(input)
  }
}

impl<I, E> Parser<I, Self, E> for BinaryType
where
  I: InputTake + Compare<[u8; 1]>,
  E: ParseError<I>,
{
  fn parse(&mut self, input: I) -> IResult<I, Self, E> {
    value(*self, tag([*self as u8]))(input)
  }
}

/// 2.1.2.3 `PrimitiveTypeEnumeration`
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum PrimitiveType {
  Boolean  = 1,
  Byte     = 2,
  Char     = 3,
  Decimal  = 5,
  Double   = 6,
  Int16    = 7,
  Int32    = 8,
  Int64    = 9,
  SByte    = 10,
  Single   = 11,
  TimeSpan = 12,
  DateTime = 13,
  UInt16   = 14,
  UInt32   = 15,
  UInt64   = 16,
  Null     = 17,
  String   = 18,
}

impl PrimitiveType {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    alt((
      Self::Boolean,
      Self::Byte,
      Self::Char,
      Self::Decimal,
      Self::Double,
      Self::Int16,
      Self::Int32,
      Self::Int64,
      Self::SByte,
      Self::Single,
      Self::TimeSpan,
      Self::DateTime,
      Self::UInt16,
      Self::UInt32,
      Self::UInt64,
      Self::Null,
      Self::String,
    ))(input)
  }
}

impl<I, E> Parser<I, Self, E> for PrimitiveType
where
  I: InputTake + Compare<[u8; 1]>,
  E: ParseError<I>,
{
  fn parse(&mut self, input: I) -> IResult<I, Self, E> {
    value(*self, tag([*self as u8]))(input)
  }
}
