//! 2.1.2 Enumerations

use nom::{
  branch::alt, bytes::complete::tag, combinator::value, error::ParseError, Compare, IResult, InputTake, Parser,
};

use crate::{
  combinator::into_failure,
  error::{error_position, Error},
};

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
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, Error<'_>> {
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
    .map_err(into_failure)
    .map_err(|err| err.map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedBinaryType)))
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
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, Error<'_>> {
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
    .map_err(into_failure)
    .map_err(|err| err.map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedPrimitiveType)))
  }

  pub(crate) fn description(&self) -> &'static str {
    match self {
      Self::Boolean => "a BOOLEAN",
      Self::Byte => "a BYTE",
      Self::Char => "a CHAR",
      Self::Decimal => "a Decimal",
      Self::Double => "a DOUBLE",
      Self::Int16 => "an INT16",
      Self::Int32 => "an INT32",
      Self::Int64 => "an INT64",
      Self::SByte => "an INT8",
      Self::Single => "a SINGLE",
      Self::TimeSpan => "a TimeSpan",
      Self::DateTime => "a DateTime",
      Self::UInt16 => "a UINT16",
      Self::UInt32 => "a UINT32",
      Self::UInt64 => "a UINT64",
      Self::Null => "a NULL",
      Self::String => "a LengthPrefixedString",
    }
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

/// 2.4.1.1 `BinaryArrayTypeEnumeration`
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum BinaryArrayType {
  Single            = 0,
  Jagged            = 1,
  Rectangular       = 2,
  SingleOffset      = 3,
  JaggedOffset      = 4,
  RectangularOffset = 5,
}

impl BinaryArrayType {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, Error<'_>> {
    alt((
      Self::Single,
      Self::Jagged,
      Self::Rectangular,
      Self::SingleOffset,
      Self::JaggedOffset,
      Self::RectangularOffset,
    ))(input)
    .map_err(into_failure)
    .map_err(|err| err.map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedBinaryArrayType)))
  }
}

impl<I, E> Parser<I, Self, E> for BinaryArrayType
where
  I: InputTake + Compare<[u8; 1]>,
  E: ParseError<I>,
{
  fn parse(&mut self, input: I) -> IResult<I, Self, E> {
    value(*self, tag([*self as u8]))(input)
  }
}
