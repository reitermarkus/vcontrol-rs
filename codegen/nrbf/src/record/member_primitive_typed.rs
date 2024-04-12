use nom::IResult;

use crate::{
  data_type::{
    Boolean, Byte, Char, DateTime, Decimal, Double, Int16, Int32, Int64, Int8, Single, TimeSpan, UInt16, UInt32, UInt64,
  },
  enumeration::PrimitiveType,
  error::Error,
  record::{MemberPrimitiveUnTyped, RecordType},
  value, Value,
};

/// 2.5.1 `MemberPrimitiveTyped`
#[derive(Debug, Clone, PartialEq)]
pub enum MemberPrimitiveTyped {
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
}

impl MemberPrimitiveTyped {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, Error<'_>> {
    let (input, _) = RecordType::MemberPrimitiveTyped.parse(input)?;

    let (input, primitive_type) = PrimitiveType::parse(input)?;
    let (input, primitive_untyped) = MemberPrimitiveUnTyped::parse(input, primitive_type)?;

    let primitive_typed = match primitive_untyped {
      MemberPrimitiveUnTyped::Boolean(v) => Self::Boolean(v),
      MemberPrimitiveUnTyped::Byte(v) => Self::Byte(v),
      MemberPrimitiveUnTyped::Char(v) => Self::Char(v),
      MemberPrimitiveUnTyped::Decimal(v) => Self::Decimal(v),
      MemberPrimitiveUnTyped::Double(v) => Self::Double(v),
      MemberPrimitiveUnTyped::Int16(v) => Self::Int16(v),
      MemberPrimitiveUnTyped::Int32(v) => Self::Int32(v),
      MemberPrimitiveUnTyped::Int64(v) => Self::Int64(v),
      MemberPrimitiveUnTyped::SByte(v) => Self::SByte(v),
      MemberPrimitiveUnTyped::Single(v) => Self::Single(v),
      MemberPrimitiveUnTyped::TimeSpan(v) => Self::TimeSpan(v),
      MemberPrimitiveUnTyped::DateTime(v) => Self::DateTime(v),
      MemberPrimitiveUnTyped::UInt16(v) => Self::UInt16(v),
      MemberPrimitiveUnTyped::UInt32(v) => Self::UInt32(v),
      MemberPrimitiveUnTyped::UInt64(v) => Self::UInt64(v),
    };

    Ok((input, primitive_typed))
  }

  #[inline]
  pub(crate) fn into_value(self) -> Value<'static> {
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
    }
  }
}
