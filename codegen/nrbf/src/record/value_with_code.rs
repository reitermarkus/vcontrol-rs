use nom::{
  branch::alt,
  combinator::{map, value},
  sequence::preceded,
  IResult,
};

use crate::{
  data_type::{
    Boolean, Byte, Char, DateTime, Decimal, Double, Int16, Int32, Int64, Int8, LengthPrefixedString, Single, TimeSpan,
    UInt16, UInt32, UInt64,
  },
  enumeration::PrimitiveType,
  value, Value,
};

/// 2.2.2.1 `ValueWithCode`
#[derive(Debug, Clone, PartialEq)]
pub enum ValueWithCode<'i> {
  Boolean(Boolean),
  Byte(Byte),
  Char(Char),
  Decimal(Decimal),
  Double(Double),
  Int16(Int16),
  Int32(Int32),
  Int64(Int64),
  SByte(Int8),
  Single(Single),
  TimeSpan(TimeSpan),
  DateTime(DateTime),
  UInt16(UInt16),
  UInt32(UInt32),
  UInt64(UInt64),
  Null,
  String(LengthPrefixedString<'i>),
}

impl<'i> ValueWithCode<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    alt((
      map(preceded(PrimitiveType::Boolean, Boolean::parse), Self::Boolean),
      map(preceded(PrimitiveType::Byte, Byte::parse), Self::Byte),
      map(preceded(PrimitiveType::Char, Char::parse), Self::Char),
      map(preceded(PrimitiveType::Decimal, Decimal::parse), Self::Decimal),
      map(preceded(PrimitiveType::Double, Double::parse), Self::Double),
      map(preceded(PrimitiveType::Int16, Int16::parse), Self::Int16),
      map(preceded(PrimitiveType::Int32, Int32::parse), Self::Int32),
      map(preceded(PrimitiveType::Int64, Int64::parse), Self::Int64),
      map(preceded(PrimitiveType::SByte, Int8::parse), Self::SByte),
      map(preceded(PrimitiveType::Single, Single::parse), Self::Single),
      map(preceded(PrimitiveType::TimeSpan, TimeSpan::parse), Self::TimeSpan),
      map(preceded(PrimitiveType::DateTime, DateTime::parse), Self::DateTime),
      map(preceded(PrimitiveType::UInt16, UInt16::parse), Self::UInt16),
      map(preceded(PrimitiveType::UInt32, UInt32::parse), Self::UInt32),
      map(preceded(PrimitiveType::UInt64, UInt64::parse), Self::UInt64),
      value(Self::Null, PrimitiveType::Null),
      map(preceded(PrimitiveType::String, LengthPrefixedString::parse), Self::String),
    ))(input)
  }

  #[inline]
  pub(crate) fn into_value(self) -> Value<'i> {
    match self {
      Self::Boolean(v) => Value::Boolean(v.into()),
      Self::Byte(v) => Value::Byte(v.into()),
      Self::Char(v) => Value::Char(v.into()),
      Self::Decimal(v) => Value::Decimal(value::Decimal(v)),
      Self::Double(v) => Value::Double(v.into()),
      Self::Int16(v) => Value::Int16(v.into()),
      Self::Int32(v) => Value::Int32(v.into()),
      Self::Int64(v) => Value::Int64(v.into()),
      Self::SByte(v) => Value::SByte(v.into()),
      Self::Single(v) => Value::Single(v.into()),
      Self::TimeSpan(v) => Value::TimeSpan(value::TimeSpan(v)),
      Self::DateTime(v) => Value::DateTime(value::DateTime(v)),
      Self::UInt16(v) => Value::UInt16(v.into()),
      Self::UInt32(v) => Value::UInt32(v.into()),
      Self::UInt64(v) => Value::UInt64(v.into()),
      Self::Null => Value::Null,
      Self::String(s) => Value::String(s.as_str()),
    }
  }
}
